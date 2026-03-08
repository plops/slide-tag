use axum::{response::Html, routing::get, Extension, Router};
use std::{net::SocketAddr, sync::Arc};
use tower_sessions::{cookie::{SameSite, time::Duration as TsDuration}, MemoryStore, Session, SessionManagerLayer, Expiry};
use serde_json::json;

use crate::{auth, db_traits::DatabaseProvider};

#[cfg(feature = "web")]
pub async fn create_app(db_provider: Arc<dyn DatabaseProvider>) -> Router {
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
        .with_expiry(Expiry::OnInactivity(TsDuration::seconds(session_max_age_secs as i64)));

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
        db_provider: db_provider.clone(),
    };

    let auth_router = auth::auth_routes();

    Router::new()
        .route("/", get(root))
        .route("/debug/session", get(debug_session))
        .nest("/auth", auth_router)
        .layer(Extension(auth_state))
        .layer(session_layer)
}

async fn root(session: Session) -> Html<String> {
    // Debug: Log session state
    println!("DEBUG root: Session ID: {:?}", session.id());

    if let Some(user_name) = session.get::<String>("user_name").await.unwrap() {
        let user_id = session.get::<i32>("user_id").await.unwrap();
        println!("DEBUG root: Found user - ID: {:?}, Name: {}", user_id, user_name);

        Html(format!(
            r#"Hello {}! Welcome to Roche Job Scraper.
            <br><br>
            <a href='/auth/logout'>Logout</a>
            <br><br>
            <p>Future features: View job matches, update profile, etc.</p>
            "#,
            user_name
        ))
    } else {
        println!("DEBUG root: No user found in session");
        Html(
            r#"Hello from Roche Job Scraper Web Server!
            <br><br>
            <a href='/auth/login'>Login with GitHub</a>
            <br><br>
            <p>Authenticate to access job matching features.</p>
            "#
            .to_string(),
        )
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
    db_provider: Arc<dyn DatabaseProvider>,
) -> anyhow::Result<()> {
    let app = create_app(db_provider).await;

    println!("Starting web server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
