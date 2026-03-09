use anyhow::Result;
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::sync::Arc;

// Import modules based on feature flags
#[cfg(feature = "db")]
use rs_scrape::{db_repo::JobRepository, db_traits::DatabaseProvider};

#[cfg(feature = "scraper")]
use rs_scrape::pipeline_orchestrator;

#[cfg(feature = "web")]
use rs_scrape::{
    ai_core::AiProvider,
    ai_gemini::GeminiProvider,
    app_state::AppState,
    config::AppConfig,
    scheduler::{NightlyScheduler, SchedulerConfig},
    web_server,
};

#[cfg(feature = "ai")]
// No additional imports needed - already imported above
#[derive(Parser)]
#[command(name = "rs-scrape")]
#[command(about = "Roche Job Scraper - Rust implementation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start webserver and background cron job
    Serve {
        #[arg(long, default_value = "3000")]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
    /// Start scraper immediately (optional debug mode)
    TriggerScrape {
        #[arg(long)]
        debug_dump: bool,
    },
    /// Force AI re-evaluation for specific candidate
    ForceMatch {
        #[arg(long)]
        candidate_id: i64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::from_env();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, host } => {
            #[cfg(not(feature = "web"))]
            compile_error!("'serve' command requires 'web' feature to be enabled. Run with: cargo run --bin main --features \"web\"");

            #[cfg(feature = "web")]
            {
                tracing::info!("Starting Roche Job Scraper web server...");

                // Initialize database
                let db_provider = init_database(&config.db_path).await?;

                // Initialize AI provider
                let ai_provider = Arc::new(GeminiProvider::new(&config.gemini_api_key)?);

                // Create app state
                let app_state = Arc::new(AppState {
                    db: db_provider.clone(),
                    ai: ai_provider,
                });

                // Use CLI parameters if provided, otherwise use config defaults
                let final_host = if host != "127.0.0.1" {
                    host
                } else {
                    config.host.clone()
                };
                let final_port = if port != 3000 { port } else { config.port };

                // Start web server
                let addr: SocketAddr = format!("{}:{}", final_host, final_port)
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid address {}: {}", final_host, e))?;

                tracing::info!("Web server will listen on: {}", addr);

                // Initialize and start scheduler
                let scheduler_config = SchedulerConfig::default();
                let scheduler = NightlyScheduler::new(scheduler_config, app_state.clone()).await?;

                // Start both server and scheduler concurrently
                let server_handle =
                    tokio::spawn(async move { web_server::run_server(addr, app_state).await });

                let scheduler_handle = tokio::spawn(async move { 
                    if let Err(e) = scheduler.start().await {
                        tracing::error!("Scheduler error: {}", e);
                    }
                    // Keep the scheduler task alive indefinitely with yielding
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                    }
                });

                // Wait for either task to complete (or error)
                tokio::select! {
                    result = server_handle => {
                        match result {
                            Ok(Ok(())) => tracing::info!("Web server shutdown gracefully"),
                            Ok(Err(e)) => tracing::error!("Web server error: {}", e),
                            Err(e) => tracing::error!("Web server task error: {}", e),
                        }
                    }
                    result = scheduler_handle => {
                        match result {
                            Ok(()) => tracing::info!("Scheduler task completed"),
                            Err(e) => tracing::error!("Scheduler task error: {}", e),
                        }
                    }
                }
            }
        }

        Commands::TriggerScrape { debug_dump } => {
            #[cfg(not(feature = "scraper"))]
            compile_error!("'trigger-scrape' command requires 'scraper' feature to be enabled. Run with: cargo run --bin main --features \"scraper\"");

            #[cfg(feature = "scraper")]
            {
                tracing::info!("Starting job scraping pipeline...");
                if debug_dump {
                    tracing::info!(
                        "Debug dump mode enabled - HTML/JSON will be saved to debug_dumps/"
                    );
                }

                // Initialize database
                let db_provider = init_database(&config.db_path).await?;

                // Run the scraping pipeline
                pipeline_orchestrator::run_pipeline(&db_provider, debug_dump).await?;

                tracing::info!("Scraping completed successfully!");
            }
        }

        Commands::ForceMatch { candidate_id } => {
            #[cfg(not(feature = "ai"))]
            compile_error!("'force-match' command requires 'ai' feature to be enabled. Run with: cargo run --bin main --features \"ai\"");

            #[cfg(feature = "ai")]
            {
                tracing::info!(
                    "Forcing AI re-evaluation for candidate ID: {}",
                    candidate_id
                );

                // Initialize database
                let db_provider = init_database(&config.db_path).await?;

                // Get candidate
                let candidate = db_provider
                    .get_candidate_by_id(candidate_id)
                    .await?
                    .ok_or_else(|| {
                        anyhow::anyhow!("Candidate with ID {} not found", candidate_id)
                    })?;

                tracing::info!("Found candidate: {}", candidate.name);

                // Get latest jobs
                let jobs = db_provider.get_latest_jobs().await?;
                if jobs.is_empty() {
                    tracing::warn!("No jobs found in database");
                    return Ok(());
                }
                tracing::info!("Found {} jobs to match against", jobs.len());

                // Initialize AI provider
                let ai_provider = GeminiProvider::new(&config.gemini_api_key)?;

                // Perform matching
                tracing::info!("Running AI matching...");
                let matches = ai_provider
                    .match_candidate(&candidate.profile_text, jobs)
                    .await?;

                // Store matches
                let matches_count = matches.len();
                for match_data in matches {
                    db_provider.insert_candidate_match(&match_data).await?;
                    tracing::info!(
                        "Stored match for job: {} (score: {})",
                        match_data.job_identifier,
                        match_data.score
                    );
                }

                tracing::info!("AI matching completed! Stored {} matches", matches_count);
            }
        }
    }

    Ok(())
}

/// Initialize database connection and return appropriate provider/repo
#[cfg(feature = "db")]
async fn init_database(db_path: &str) -> Result<Arc<JobRepository>> {
    use rs_scrape::db_setup;

    tracing::info!("Initializing database...");

    // Setup database
    let conn = db_setup::init_db(db_path).await?;
    let repo = JobRepository::new(conn);

    tracing::info!("Database initialized: {}", db_path);
    Ok(Arc::new(repo))
}

/// Database initialization stub when db feature is not enabled
#[cfg(not(feature = "db"))]
async fn init_database(_db_path: &str) -> Result<()> {
    Err(anyhow::anyhow!("Database feature not enabled"))
}
