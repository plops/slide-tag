use axum::{
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{db_traits::DatabaseProvider, models::Candidate};

#[cfg(feature = "web")]
#[derive(Clone)]
pub struct AuthState {
    pub oauth_client: std::sync::Arc<BasicClient>,
    pub db_provider: std::sync::Arc<dyn DatabaseProvider>,
}

#[derive(Deserialize)]
struct AuthCallback {
    code: String,
    // CSRF state token returned by OAuth provider
    state: Option<String>,
}

#[cfg(feature = "web")]
pub fn create_github_oauth_client(
    client_id: String,
    client_secret: String,
    redirect_url: String,
) -> BasicClient {
    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

#[cfg(feature = "web")]
pub fn auth_routes() -> Router<()> {
    Router::new()
        .route("/login", get(login))
        .route("/callback", get(auth_callback))
        .route("/logout", get(logout))
}

async fn login(Extension(state): Extension<AuthState>, session: Session) -> impl IntoResponse {
    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    // Store CSRF token in session for validation
    session
        .insert("oauth_csrf", csrf_token.secret())
        .await
        .unwrap();

    // Debug: Verify CSRF token was stored
    println!(
        "DEBUG login: Session ID: {:?}, stored CSRF token",
        session.id()
    );

    // Try to force save (some session backends support explicit save)
    let _ = session.save().await;

    Redirect::to(auth_url.as_ref())
}

async fn auth_callback(
    Extension(state): Extension<AuthState>,
    Query(params): Query<AuthCallback>,
    session: Session,
) -> impl IntoResponse {
    // Attempt CSRF validation if present
    if let Some(state_param) = params.state.clone() {
        if let Some(stored_csrf) = session.get::<String>("oauth_csrf").await.unwrap() {
            if stored_csrf != state_param {
                eprintln!(
                    "CSRF validation failed: expected {}, got {}",
                    stored_csrf, state_param
                );
                return Redirect::to("/?error=csrf");
            }
            // Clear the CSRF token from session after successful validation
            let _ = session.remove::<String>("oauth_csrf").await;
        } else {
            eprintln!("No CSRF token found in session");
            return Redirect::to("/?error=no_csrf");
        }
    } else {
        // If provider didn't send state, log for debugging but continue for now
        println!("DEBUG auth_callback: no state parameter provided by provider");
    }

    // Exchange code for token
    let token_result = state
        .oauth_client
        .exchange_code(oauth2::AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            // Get user info from GitHub
            let client = reqwest::Client::new();
            let user_response = client
                .get("https://api.github.com/user")
                .bearer_auth(token.access_token().secret())
                .header("User-Agent", "Roche-Job-Scraper")
                .send()
                .await;

            match user_response {
                Ok(resp) => {
                    if let Ok(user_json) = resp.json::<serde_json::Value>().await {
                        let oauth_sub = user_json["id"].as_i64().unwrap().to_string();
                        let name = user_json["name"].as_str().unwrap_or("Unknown").to_string();

                        // Upsert candidate
                        let candidate = Candidate {
                            id: None,
                            oauth_sub: oauth_sub.clone(),
                            name: name.clone(),
                            profile_text: "".to_string(), // Will be filled later in profile
                        };

                        match state.db_provider.upsert_candidate(&candidate).await {
                            Ok(candidate_id) => {
                                // Debug: Log session ID before insert
                                println!("DEBUG: Session ID before insert: {:?}", session.id());

                                // Store user_id in session to maintain login state
                                session.insert("user_id", candidate_id).await.unwrap();
                                session.insert("user_name", name.clone()).await.unwrap();

                                // Debug: Log session ID after insert and verify values
                                println!("DEBUG: Session ID after insert: {:?}", session.id());
                                println!(
                                    "DEBUG: Stored user_id: {}, user_name: {}",
                                    candidate_id, name
                                );

                                // Force session save
                                if let Err(e) = session.save().await {
                                    eprintln!("ERROR: Failed to save session: {:?}", e);
                                }

                                Redirect::to("/")
                            }
                            Err(e) => {
                                eprintln!("Failed to upsert candidate: {:?}", e);
                                Redirect::to("/?error=database")
                            }
                        }
                    } else {
                        Redirect::to("/?error=parse")
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get user info: {:?}", e);
                    Redirect::to("/?error=user_info")
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to exchange code: {:?}", e);
            Redirect::to("/?error=auth")
        }
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/")
}
