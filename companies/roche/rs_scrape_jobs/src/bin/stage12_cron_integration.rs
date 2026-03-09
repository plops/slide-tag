use anyhow::Result;
use rs_scrape::scheduler::{NightlyScheduler, SchedulerConfig};
use tokio::time::{sleep, Duration};

#[cfg(feature = "web")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Stage 12 Cron Scheduler Integration Test");

    // Create a test configuration that runs every 10 seconds
    let config = SchedulerConfig {
        cron_schedule: "0/10 * * * * *".to_string(), // Every 10 seconds
        debug: true,
        max_candidate_batch_size: 2,
        batch_delay_seconds: 2,
    };

    println!("Creating scheduler with 10-second interval for testing...");

    // Create the scheduler
    let scheduler = NightlyScheduler::new(config.clone()).await?;

    // Start the scheduler
    println!("Starting scheduler...");
    scheduler.start().await?;

    println!("Scheduler started. Will run for 45 seconds to demonstrate cron functionality.");

    // Let it run for 45 seconds (should execute ~4-5 times)
    sleep(Duration::from_secs(45)).await;

    // Stop the scheduler
    println!("Stopping scheduler...");
    scheduler.stop().await?;

    println!("Test completed successfully!");

    Ok(())
}

#[cfg(not(feature = "web"))]
fn main() {
    eprintln!("This test requires the 'web' feature for tokio-cron-scheduler");
    eprintln!("Run with: cargo run --features \"web\" --bin stage12_cron_integration");
    std::process::exit(1);
}
