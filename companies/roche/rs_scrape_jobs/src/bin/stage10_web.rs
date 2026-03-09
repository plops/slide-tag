use rs_scrape::{app_state::AppState, ai_gemini::GeminiProvider, db_repo, db_setup, web_server};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting stage 10: Axum Web Server & Authentication");

    // Initialize database
    let conn = db_setup::init_db("jobs_minutils.db").await?;
    let db_repo = Arc::new(db_repo::JobRepository::new(conn));
    
    // Initialize AI provider
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| "test_key".to_string());
    let ai_provider = Arc::new(GeminiProvider::new(&api_key)?);
    
    // Create app state
    let app_state = Arc::new(AppState {
        db: db_repo,
        ai: ai_provider,
    });

    // Start web server on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    web_server::run_server(addr, app_state).await?;

    Ok(())
}
