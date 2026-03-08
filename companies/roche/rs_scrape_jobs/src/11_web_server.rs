use axum::{routing::get, Router, Extension, response::Html};
use std::{net::SocketAddr, sync::Arc};
use tower_sessions::{MemoryStore, SessionManagerLayer, Session};

use crate::{auth, db_traits::DatabaseProvider};

#[cfg(feature = "web")]
pub async fn create_app(db_provider: Arc<dyn DatabaseProvider>) -> Router {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

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
        .nest("/auth", auth_router)
        .layer(Extension(auth_state))
        .layer(session_layer)
}

async fn root(session: Session) -> Html<String> {
    if let Some(user_name) = session.get::<String>("user_name").await.unwrap() {
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
        Html(r#"Hello from Roche Job Scraper Web Server!
            <br><br>
            <a href='/auth/login'>Login with GitHub</a>
            <br><br>
            <p>Authenticate to access job matching features.</p>
            "#.to_string())
    }
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
