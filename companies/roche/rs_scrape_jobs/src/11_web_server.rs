use axum::{
    response::{Html, Redirect},
    routing::{get, post},
    Extension, Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tower_sessions::{
    cookie::{time::Duration as TsDuration, SameSite},
    Expiry, MemoryStore, Session, SessionManagerLayer,
};

use crate::{app_state::AppState, auth, web_ui};

#[cfg(feature = "web")]
pub async fn create_app(app_state: Arc<AppState>) -> Router {
    let session_store = MemoryStore::default();

    // Configure session cookie using environment variables where appropriate
    let session_secure = std::env::var("SESSION_SECURE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    let session_max_age_days = std::env::var("SESSION_MAX_AGE_DAYS")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u64>()
        .unwrap_or(30);

    let session_max_age_secs = session_max_age_days * 24 * 60 * 60;

    // Optional domain for cookie (useful in production)
    let session_domain = std::env::var("SESSION_DOMAIN").ok();

    // Create session layer and apply configuration per DeepWiki
    let mut session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_secure(session_secure)
        .with_path("/")
        .with_name("id")
        .with_expiry(Expiry::OnInactivity(TsDuration::seconds(
            session_max_age_secs as i64,
        )));

    if let Some(domain) = session_domain {
        // Move domain into layer if provided
        session_layer = session_layer.with_domain(domain);
    }

    let client_id = std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let client_secret =
        std::env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set");
    let redirect_url = std::env::var("OAUTH_REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string());

    let oauth_client = auth::create_github_oauth_client(client_id, client_secret, redirect_url);
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
        .route("/api/trigger-match", post(web_ui::trigger_match))
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

async fn root(session: Session) -> Redirect {
    // Debug: Log session state
    println!("DEBUG root: Session ID: {:?}", session.id());

    if let Some(_user_name) = session.get::<String>("user_name").await.unwrap() {
        let _user_id = session.get::<i64>("user_id").await.unwrap();
        println!(
            "DEBUG root: Found user - ID: {:?}, Name: {}",
            _user_id, _user_name
        );
        // Redirect authenticated users to dashboard
        Redirect::to("/dashboard")
    } else {
        println!("DEBUG root: No user found in session");
        // Redirect unauthenticated users to login
        Redirect::to("/auth/login")
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
pub async fn run_server(addr: SocketAddr, app_state: Arc<AppState>) -> anyhow::Result<()> {
    let app = create_app(app_state).await;

    println!("Starting web server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
