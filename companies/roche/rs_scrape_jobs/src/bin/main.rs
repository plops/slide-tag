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
    app_state::AppState, ai_core::AiProvider, ai_gemini::GeminiProvider, scheduler::{NightlyScheduler, SchedulerConfig},
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, host } => {
            #[cfg(not(feature = "web"))]
            compile_error!("'serve' command requires 'web' feature to be enabled. Run with: cargo run --bin main --features \"web\"");

            #[cfg(feature = "web")]
            {
                println!("Starting Roche Job Scraper web server...");

                // Initialize database
                let db_provider = init_database().await?;
                
                // Initialize AI provider
                let api_key = std::env::var("GEMINI_API_KEY")
                    .map_err(|_| anyhow::anyhow!("GEMINI_API_KEY environment variable not set"))?;
                let ai_provider = Arc::new(GeminiProvider::new(&api_key)?);
                
                // Create app state
                let app_state = Arc::new(AppState {
                    db: db_provider.clone(),
                    ai: ai_provider,
                });

                // Start web server
                let addr: SocketAddr = format!("{}:{}", host, port)
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid address {}: {}", host, e))?;

                println!("Web server will listen on: {}", addr);

                // Initialize and start scheduler
                let scheduler_config = SchedulerConfig::default();
                let scheduler = NightlyScheduler::new(scheduler_config).await?;

                // Start both server and scheduler concurrently
                let server_handle =
                    tokio::spawn(async move { web_server::run_server(addr, app_state).await });

                let scheduler_handle = tokio::spawn(async move { scheduler.start().await });

                // Wait for either task to complete (or error)
                tokio::select! {
                    result = server_handle => {
                        match result {
                            Ok(Ok(())) => println!("Web server shutdown gracefully"),
                            Ok(Err(e)) => eprintln!("Web server error: {}", e),
                            Err(e) => eprintln!("Web server task error: {}", e),
                        }
                    }
                    result = scheduler_handle => {
                        match result {
                            Ok(Ok(())) => println!("Scheduler shutdown gracefully"),
                            Ok(Err(e)) => eprintln!("Scheduler error: {}", e),
                            Err(e) => eprintln!("Scheduler task error: {}", e),
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
                println!("Starting job scraping pipeline...");
                if debug_dump {
                    println!("Debug dump mode enabled - HTML/JSON will be saved to debug_dumps/");
                }

                // Initialize database
                let db_provider = init_database().await?;

                // Run the scraping pipeline
                pipeline_orchestrator::run_pipeline(&db_provider, debug_dump).await?;

                println!("Scraping completed successfully!");
            }
        }

        Commands::ForceMatch { candidate_id } => {
            #[cfg(not(feature = "ai"))]
            compile_error!("'force-match' command requires 'ai' feature to be enabled. Run with: cargo run --bin main --features \"ai\"");

            #[cfg(feature = "ai")]
            {
                println!(
                    "Forcing AI re-evaluation for candidate ID: {}",
                    candidate_id
                );

                // Initialize database
                let db_provider = init_database().await?;

                // Get candidate
                let candidate = db_provider
                    .get_candidate_by_id(candidate_id)
                    .await?
                    .ok_or_else(|| {
                        anyhow::anyhow!("Candidate with ID {} not found", candidate_id)
                    })?;

                println!("Found candidate: {}", candidate.name);

                // Get latest jobs
                let jobs = db_provider.get_latest_jobs().await?;
                if jobs.is_empty() {
                    println!("No jobs found in database");
                    return Ok(());
                }
                println!("Found {} jobs to match against", jobs.len());

                // Initialize AI provider
                let api_key = std::env::var("GEMINI_API_KEY")
                    .map_err(|_| anyhow::anyhow!("GEMINI_API_KEY environment variable not set"))?;
                let ai_provider = GeminiProvider::new(&api_key)?;

                // Perform matching
                println!("Running AI matching...");
                let matches = ai_provider
                    .match_candidate(&candidate.profile_text, jobs)
                    .await?;

                // Store matches
                let matches_count = matches.len();
                for match_data in matches {
                    db_provider.insert_candidate_match(&match_data).await?;
                    println!(
                        "Stored match for job: {} (score: {})",
                        match_data.job_identifier, match_data.score
                    );
                }

                println!("AI matching completed! Stored {} matches", matches_count);
            }
        }
    }

    Ok(())
}

/// Initialize database connection and return appropriate provider/repo
#[cfg(feature = "db")]
async fn init_database() -> Result<Arc<JobRepository>> {
    use rs_scrape::db_setup;

    println!("Initializing database...");

    // Setup database
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "jobs_minutils.db".to_string());

    let conn = db_setup::init_db(&db_path).await?;
    let repo = JobRepository::new(conn);

    println!("Database initialized: {}", db_path);
    Ok(Arc::new(repo))
}

/// Database initialization stub when db feature is not enabled
#[cfg(not(feature = "db"))]
async fn init_database() -> Result<()> {
    Err(anyhow::anyhow!("Database feature not enabled"))
}
