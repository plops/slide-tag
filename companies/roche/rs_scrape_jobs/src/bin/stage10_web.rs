use rs_scrape::{db_repo, db_setup, web_server};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting stage 10: Axum Web Server & Authentication");

    // Initialize database
    let conn = db_setup::init_db("jobs_minutils.db").await?;
    let repo = Arc::new(db_repo::JobRepository::new(conn));

    // Start web server on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    web_server::run_server(addr, repo).await?;

    Ok(())
}
