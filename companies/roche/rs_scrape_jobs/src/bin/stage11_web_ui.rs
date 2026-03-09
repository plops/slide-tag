use rs_scrape::{
    ai_gemini::GeminiProvider, app_state::AppState, db_repo::JobRepository, db_setup::init_db,
    web_server,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    println!("🚀 Starting Stage 11 Web UI Server...");

    // Set up database
    let conn = init_db("jobs_stage11.db").await?;
    let repo = JobRepository::new(conn);
    let db_provider = Arc::new(repo);

    // Initialize AI provider (mock for testing)
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| "test_key".to_string());
    let ai_provider = Arc::new(GeminiProvider::new(&api_key)?);

    // Create app state
    let app_state = Arc::new(AppState {
        db: db_provider,
        ai: ai_provider,
    });

    // Create the app
    let app = web_server::create_app(app_state).await;

    // Bind to port 3000 (standard port for OAuth callback)
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Stage 11 Web UI server running on http://127.0.0.1:3000");
    println!("📝 Features: Profile management, Dashboard with job matches");
    println!("🔐 Auth: GitHub OAuth integration");
    println!("🎨 Templates: Askama-based HTML rendering");

    // Run the server
    axum::serve(listener, app).await?;

    Ok(())
}
