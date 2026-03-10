use axum::{
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tower_sessions::{
    cookie::{time::Duration as TsDuration, SameSite},
    Expiry, Session, SessionManagerLayer,
};

use crate::{
    admin, app_state::AppState, auth, config::AppConfig, custom_session_store::LibsqlSessionStore,
    web_ui,
};

#[cfg(feature = "web")]
pub async fn create_app(app_state: Arc<AppState>, config: &AppConfig) -> Router {
    // Use custom LibsqlSessionStore for persistent sessions
    let session_store = LibsqlSessionStore::new(app_state.db.clone());

    // Run migration to create sessions table
    session_store
        .migrate()
        .await
        .expect("Failed to migrate session store table");

    // Start background cleanup task
    let cleanup_store = session_store.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Every hour
        loop {
            interval.tick().await;
            if let Err(e) = cleanup_store.cleanup_expired().await {
                eprintln!("Failed to cleanup expired sessions: {}", e);
            }
        }
    });

    // Configure session cookie using config values
    let session_max_age_secs = config.session_max_age_days * 24 * 60 * 60;

    // Optional domain for cookie (useful in production)
    let session_domain = std::env::var("SESSION_DOMAIN").ok();

    // Create session layer and apply configuration per DeepWiki
    let mut session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_secure(config.session_secure)
        .with_path("/")
        .with_name("id")
        .with_expiry(Expiry::OnInactivity(TsDuration::seconds(
            session_max_age_secs as i64,
        )));

    if let Some(domain) = session_domain {
        // Move domain into layer if provided
        session_layer = session_layer.with_domain(domain);
    }

    let oauth_client = auth::create_github_oauth_client(
        config.github_client_id.clone(),
        config.github_client_secret.clone(),
        config.oauth_redirect_url.clone(),
    );
    let auth_state = auth::AuthState {
        oauth_client: std::sync::Arc::new(oauth_client),
        db_provider: app_state.db.clone(),
    };

    let auth_router = auth::auth_routes();

    // Router for routes that need database state
    let db_routes = Router::new()
        .route(
            "/profile",
            get(web_ui::get_profile).post(web_ui::post_profile),
        )
        .route("/dashboard", get(web_ui::get_dashboard))
        .route("/match/{id}", get(web_ui::get_match_detail))
        .route("/job/{identifier}", get(web_ui::get_job_detail))
        .route("/jobs", get(web_ui::get_jobs))
        .route("/api/trigger-match", post(web_ui::trigger_match))
        .nest("/admin", admin::admin_routes())
        .with_state(app_state.clone());

    // Main router with session and auth layers applied to everything
    Router::new()
        .route("/", get(root))
        .route("/debug/session", get(debug_session))
        .nest("/auth", auth_router)
        .merge(db_routes)
        .layer(Extension(auth_state))
        .layer(session_layer)
}

async fn root(session: Session) -> Result<Html<String>, axum::response::ErrorResponse> {
    use askama::Template;

    // Debug: Log session state
    println!("DEBUG root: Session ID: {:?}", session.id());

    let user_name = session.get::<String>("user_name").await.unwrap_or(None);

    if let Some(ref name) = user_name {
        let _user_id = session.get::<i64>("user_id").await.unwrap();
        println!(
            "DEBUG root: Found user - ID: {:?}, Name: {}",
            _user_id, name
        );
    } else {
        println!("DEBUG root: No user found in session");
    }

    // Render index template
    #[derive(askama::Template)]
    #[template(path = "index.html")]
    struct IndexTemplate {
        user_name: String,
        app_version: &'static str,
    }

    let template = IndexTemplate {
        user_name: user_name.unwrap_or_else(|| "Guest".to_string()),
        app_version: env!("CARGO_PKG_VERSION"),
    };
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(e) => {
            eprintln!("Failed to render index template: {:?}", e);
            Err(axum::response::ErrorResponse::from((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Template error",
            )))
        }
    }
}

// New debug endpoint to inspect session content
async fn debug_session(session: Session) -> Html<String> {
    let mut map = serde_json::Map::new();
    // Safely serialize session id: turn into string or null to avoid serde errors panicking
    let session_id_value = match session.id() {
        Some(s) => serde_json::Value::String(s.to_string()),
        None => serde_json::Value::Null,
    };
    map.insert("session_id".to_string(), session_id_value);

    // Attempt to read a few known keys
    if let Ok(Some(user_id)) = session.get::<i32>("user_id").await {
        map.insert("user_id".to_string(), json!(user_id));
    }
    if let Ok(Some(user_name)) = session.get::<String>("user_name").await {
        map.insert("user_name".to_string(), json!(user_name));
    }
    if let Ok(Some(oauth_csrf)) = session.get::<String>("oauth_csrf").await {
        map.insert("oauth_csrf".to_string(), json!(oauth_csrf));
    }

    Html(serde_json::Value::Object(map).to_string())
}

#[cfg(feature = "web")]
pub async fn run_server(
    addr: SocketAddr,
    app_state: Arc<AppState>,
    config: &AppConfig,
) -> anyhow::Result<()> {
    let app = create_app(app_state, config).await;

    println!("Starting web server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
