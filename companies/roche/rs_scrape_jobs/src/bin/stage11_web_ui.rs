use rs_scrape::db_repo::JobRepository;
use rs_scrape::db_setup::init_db;
use rs_scrape::web_server;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    println!("🚀 Starting Stage 11 Web UI Server...");

    // Set up database
    let conn = init_db("jobs_stage11.db").await?;
    let repo = JobRepository::new(conn);

    // Create web server
    let db_provider = Arc::new(repo);

    // Create the app
    let app = web_server::create_app(db_provider).await;

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
