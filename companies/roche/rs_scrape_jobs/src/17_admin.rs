use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, Redirect},
};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;
use tracing;

use crate::{
    app_state::{AppState, ScrapeStatus},
    pipeline_orchestrator,
    web_ui::WebError,
};

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminTemplate {
    pub title: String,
    pub user_name: String,
    pub status: String,
    pub is_running: bool,
    pub app_version: &'static str,
}

#[derive(Deserialize)]
pub struct TriggerScrapeForm {
    #[serde(default)]
    pub debug_dump: String, // Checkbox sendet "on" oder existiert nicht
}

// Helfer zur Admin-Verifizierung
async fn is_admin(session: &Session, state: &AppState) -> Result<String, WebError> {
    let user_name = session
        .get::<String>("user_name")
        .await
        .map_err(|_| WebError::Auth("Session error".to_string()))?
        .unwrap_or_default();

    if user_name.to_lowercase() != state.config.admin_username.to_lowercase() {
        return Err(WebError::Auth("Access Denied: Admin only".to_string()));
    }
    Ok(user_name)
}

pub async fn get_admin_dashboard(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    let user_name = is_admin(&session, &state).await?;

    let status_lock = state.scrape_status.read().await;
    let (status_text, is_running) = match &*status_lock {
        ScrapeStatus::Idle => ("Idle (Ready to scrape)".to_string(), false),
        ScrapeStatus::Running {
            start_time,
            debug_mode,
        } => (
            format!(
                "Running since {} (Debug: {})",
                start_time.format("%H:%M:%S"),
                debug_mode
            ),
            true,
        ),
        ScrapeStatus::Success(time) => (
            format!("Last success at {}", time.format("%H:%M:%S")),
            false,
        ),
        ScrapeStatus::Error(msg) => (format!("Error: {}", msg), false),
    };

    let template = AdminTemplate {
        title: "Admin Dashboard".to_string(),
        user_name,
        status: status_text,
        is_running,
        app_version: env!("CARGO_PKG_VERSION"),
    };

    Ok(Html(template.render()?))
}

pub async fn post_trigger_scrape(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<TriggerScrapeForm>,
) -> Result<Redirect, WebError> {
    is_admin(&session, &state).await?;

    let debug_dump = form.debug_dump == "on";

    // Status prüfen und setzen
    {
        let mut status_lock = state.scrape_status.write().await;
        if let ScrapeStatus::Running { .. } = *status_lock {
            return Ok(Redirect::to("/admin?error=already_running"));
        }
        *status_lock = ScrapeStatus::Running {
            start_time: Utc::now(),
            debug_mode: debug_dump,
        };
    }

    // Task im Hintergrund starten! Fire and forget.
    let bg_state = state.clone();
    tokio::spawn(async move {
        tracing::info!("Admin triggered manual scrape. Debug: {}", debug_dump);

        match pipeline_orchestrator::run_pipeline(bg_state.db.clone(), Some(bg_state.ai.clone()), debug_dump).await {
            Ok(_) => {
                tracing::info!("Manual scrape completed successfully.");
                let mut status_lock = bg_state.scrape_status.write().await;
                *status_lock = ScrapeStatus::Success(Utc::now());
            }
            Err(e) => {
                tracing::error!("Manual scrape failed: {:?}", e);
                let mut status_lock = bg_state.scrape_status.write().await;
                *status_lock = ScrapeStatus::Error(e.to_string());
            }
        }
    });

    Ok(Redirect::to("/admin"))
}

pub async fn post_trigger_ai(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, WebError> {
    is_admin(&session, &state).await?;

    let bg_state = state.clone();
    tokio::spawn(async move {
        tracing::info!("Admin hat manuelle AI-Annotation getriggert.");
        
        #[cfg(feature = "ai")]
        match crate::ai_workflow::annotate_unannotated_jobs(bg_state.db.clone(), bg_state.ai.clone(), 50).await {
            Ok(count) => tracing::info!("Admin-Action: Erfolgreich {} Jobs annotiert.", count),
            Err(e) => tracing::error!("Admin-Action: AI Annotation fehlgeschlagen: {:?}", e),
        }
    });

    Ok(Redirect::to("/admin"))
}

pub fn admin_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", axum::routing::get(get_admin_dashboard))
        .route("/trigger", axum::routing::post(post_trigger_scrape))
        .route("/trigger-ai", axum::routing::post(post_trigger_ai))
}
