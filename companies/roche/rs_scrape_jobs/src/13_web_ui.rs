use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_sessions::Session;

use crate::{
    app_state::AppState, db_traits::DatabaseProvider, models::Candidate, models::CandidateMatch, models::JobHistory,
};

// Template structs for Askama

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub title: String,
    pub user_name: String,
    pub profile_text: String,
    pub success: bool,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub title: String,
    pub user_name: String,
    pub matches_count: usize,
    pub high_score_count: usize,
    pub good_fit_count: usize,
    pub has_matches: bool,
}

// Data structure for dashboard - simplified for Askama
#[derive(Serialize)]
pub struct MatchWithJob {
    pub match_data: CandidateMatch,
    pub job: Option<JobHistory>,
    pub match_score_percent: i32,
    pub match_date_formatted: String,
}

// Form data structures
#[derive(Deserialize)]
pub struct ProfileForm {
    pub profile_text: String,
}

// Error handling
#[derive(Debug)]
pub enum WebError {
    Database(anyhow::Error),
    Template(askama::Error),
    Auth(String),
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::Database(e) => write!(f, "Database error: {}", e),
            WebError::Template(e) => write!(f, "Template error: {}", e),
            WebError::Auth(msg) => write!(f, "Authentication error: {}", msg),
        }
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            WebError::Database(e) => {
                log::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error occurred".to_string(),
                )
            }
            WebError::Template(e) => {
                log::error!("Template error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Template rendering error".to_string(),
                )
            }
            WebError::Auth(msg) => {
                log::warn!("Authentication error: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head><title>Error</title></head>
<body>
    <h1>Error</h1>
    <p>{}</p>
    <a href="/">Go to Home</a>
</body>
</html>"#,
            error_message
        );

        (status, Html(html)).into_response()
    }
}

impl From<anyhow::Error> for WebError {
    fn from(err: anyhow::Error) -> Self {
        WebError::Database(err)
    }
}

impl From<askama::Error> for WebError {
    fn from(err: askama::Error) -> Self {
        WebError::Template(err)
    }
}

// Helper functions
async fn get_current_user(
    session: &Session,
    db_provider: &dyn DatabaseProvider,
) -> Result<Candidate, WebError> {
    let oauth_sub = session
        .get::<String>("oauth_sub")
        .await
        .map_err(|_| WebError::Auth("Failed to get oauth_sub from session".to_string()))?
        .ok_or_else(|| WebError::Auth("No oauth_sub in session".to_string()))?;

    db_provider
        .get_candidate_by_oauth_sub(&oauth_sub)
        .await
        .map_err(WebError::Database)?
        .ok_or_else(|| WebError::Auth("Candidate not found".to_string()))
}

// Route handlers
pub async fn get_profile(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Get current user
    let candidate = get_current_user(&session, &*state.db).await?;

    let success = session
        .get::<bool>("success")
        .await
        .unwrap_or(None)
        .unwrap_or(false);

    // Clear the success flag
    let _ = session.remove_value("success").await;

    let template = ProfileTemplate {
        title: "Profile".to_string(),
        user_name: candidate.name,
        profile_text: candidate.profile_text,
        success,
    };

    Ok(Html(template.render()?))
}

pub async fn post_profile(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<ProfileForm>,
) -> Result<Redirect, WebError> {
    // Get current user
    let candidate = get_current_user(&session, &*state.db).await?;

    // Update candidate profile in database
    let updated_candidate = Candidate {
        id: candidate.id,
        oauth_sub: candidate.oauth_sub,
        name: candidate.name,
        profile_text: form_data.profile_text.clone(),
    };

    state.db
        .upsert_candidate(&updated_candidate)
        .await
        .map_err(WebError::Database)?;

    // Set success flag in session
    session
        .insert("success", true)
        .await
        .map_err(|_| WebError::Auth("Failed to set success flag".to_string()))?;

    log::info!("Profile updated for user: {}", updated_candidate.oauth_sub);

    // Redirect back to profile
    Ok(Redirect::to("/profile"))
}

pub async fn get_dashboard(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Get current user
    let candidate = get_current_user(&session, &*state.db).await?;

    // Get matches for this candidate
    let matches = state.db
        .get_matches_for_candidate(candidate.id.unwrap_or(0))
        .await
        .map_err(WebError::Database)?;

    // Calculate statistics
    let matches_count = matches.len();
    let high_score_count = matches.iter().filter(|m| m.score > 0.8).count();
    let good_fit_count = matches.iter().filter(|m| m.score > 0.6).count();
    let has_matches = matches_count > 0;

    let template = DashboardTemplate {
        title: "Dashboard".to_string(),
        user_name: candidate.name,
        matches_count,
        high_score_count,
        good_fit_count,
        has_matches,
    };

    Ok(Html(template.render()?))
}

pub async fn trigger_match(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, WebError> {
    let candidate = get_current_user(&session, &*state.db).await?;
    let candidate_id = candidate.id.unwrap_or(0);
    
    // Klone den State für den Hintergrund-Task
    let bg_state = state.clone();
    let profile_text = candidate.profile_text.clone();
    let oauth_sub = candidate.oauth_sub.clone();
    
    // Background Task (Fire and Forget)
    tokio::spawn(async move {
        log::info!("Starte asynchrone KI-Evaluierung für User: {}", oauth_sub);
        
        // 1. Hole aktuelle Jobs
        if let Ok(jobs) = bg_state.db.get_latest_jobs().await {
            // 2. Starte KI Matching
            if let Ok(matches) = bg_state.ai.match_candidate(&profile_text, jobs).await {
                // 3. Speichere Ergebnisse
                for match_data in matches {
                    let mut final_match = match_data.clone();
                    final_match.candidate_id = candidate_id;
                    let _ = bg_state.db.insert_candidate_match(&final_match).await;
                }
                log::info!("KI-Evaluierung für User {} abgeschlossen.", oauth_sub);
            }
        }
    });

    // Setze eine Flash-Message für das UI (optional, falls implementiert)
    let _ = session.insert("success", true).await;
    
    // Sofortiger Redirect, während die KI im Hintergrund rechnet
    Ok(Redirect::to("/dashboard"))
}

// Router configuration
pub fn web_ui_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/profile", get(get_profile).post(post_profile))
        .route("/dashboard", get(get_dashboard))
        .route("/api/trigger-match", post(trigger_match))
}
