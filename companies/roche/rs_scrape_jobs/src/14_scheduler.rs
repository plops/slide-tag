use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job as CronJob, JobScheduler, JobSchedulerError};

/// Configuration for the nightly job scheduler
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Cron expression for when to run the job (default: "0 2 * * *" for 2:00 AM daily)
    pub cron_schedule: String,
    /// Enable debug mode with verbose logging
    pub debug: bool,
    /// Maximum number of candidates to process in one batch
    pub max_candidate_batch_size: usize,
    /// Delay between candidate processing batches (in seconds)
    pub batch_delay_seconds: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            cron_schedule: "0 2 * * *".to_string(), // 2:00 AM daily
            debug: false,
            max_candidate_batch_size: 5,
            batch_delay_seconds: 30,
        }
    }
}

/// Main scheduler that orchestrates the nightly job scraping and AI matching
pub struct NightlyScheduler {
    scheduler: Arc<Mutex<JobScheduler>>,
    config: SchedulerConfig,
}

impl NightlyScheduler {
    /// Create a new scheduler instance
    pub async fn new(config: SchedulerConfig) -> Result<Self, JobSchedulerError> {
        let scheduler = JobScheduler::new().await?;

        Ok(Self {
            scheduler: Arc::new(Mutex::new(scheduler)),
            config,
        })
    }

    /// Start the scheduler and begin executing jobs
    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        let config = self.config.clone();

        // Create the main nightly job
        let job = CronJob::new_async(config.cron_schedule.clone(), move |_uuid, _l| {
            let config = config.clone();

            Box::pin(async move {
                if let Err(e) = execute_nightly_pipeline(config).await {
                    eprintln!("Nightly pipeline failed: {}", e);
                    // In production, you might want to send an alert here
                }
            })
        })?;

        // Add the job to the scheduler
        {
            let scheduler = self.scheduler.lock().await;
            scheduler.add(job).await?;
        }

        // Start the scheduler
        {
            let scheduler = self.scheduler.lock().await;
            scheduler.start().await?;
        }

        println!(
            "Nightly scheduler started with schedule: {}",
            self.config.cron_schedule
        );
        Ok(())
    }

    /// Stop the scheduler gracefully
    pub async fn stop(&self) -> Result<(), JobSchedulerError> {
        let mut scheduler = self.scheduler.lock().await;
        scheduler.shutdown().await?;
        println!("Nightly scheduler stopped");
        Ok(())
    }

    /// Trigger the pipeline manually for testing
    pub async fn trigger_manual(&self) -> Result<()> {
        println!("Manually triggering nightly pipeline...");
        execute_nightly_pipeline(self.config.clone()).await
    }
}

/// Execute the complete nightly pipeline
async fn execute_nightly_pipeline(config: SchedulerConfig) -> Result<()> {
    let start_time = Utc::now();
    println!("Starting nightly pipeline at {}", start_time);

    // Step 1: Scrape new jobs
    println!("Step 1: Scraping jobs from Roche...");
    scrape_jobs_and_store(config.debug).await?;

    // Step 2: Get all candidates for matching
    println!("Step 2: Fetching candidates for AI matching...");
    let candidates = get_all_candidates().await?;

    if candidates.is_empty() {
        println!("No candidates found, skipping AI matching");
        return Ok(());
    }

    println!("Found {} candidates to process", candidates.len());

    // Step 3: Get latest jobs for matching
    println!("Step 3: Fetching latest jobs for matching...");
    let jobs = get_latest_jobs().await?;

    if jobs.is_empty() {
        println!("No jobs found, skipping AI matching");
        return Ok(());
    }

    println!("Found {} jobs to match against", jobs.len());

    // Step 4: Process candidates in batches with rate limiting
    println!("Step 4: Processing AI matches with rate limiting...");
    process_candidate_matches(candidates, jobs, &config).await?;

    let end_time = Utc::now();
    let duration = end_time - start_time;
    println!("Nightly pipeline completed successfully in {}", duration);

    Ok(())
}

/// Scrape jobs and store them in the database
async fn scrape_jobs_and_store(debug: bool) -> Result<()> {
    println!("Job scraping would be executed here");
    println!("Debug mode: {}", debug);

    // In the actual implementation, this would:
    // 1. Setup browser using web_core::setup_browser()
    // 2. Scrape URLs using scraper_roche::scrape_roche_jobs()
    // 3. Download pages using downloader::download_pages()
    // 4. Extract JSON using json_extractor::extract_phapp_json_regex()
    // 5. Parse jobs using data_ingestion::parse_roche_job()
    // 6. Store in database using DatabaseProvider::insert_job_history()

    Ok(())
}

/// Get all candidates from the database
async fn get_all_candidates() -> Result<Vec<String>> {
    // For now, return mock candidate IDs
    // In the actual implementation, this would use DatabaseProvider::get_all_candidates()
    println!("Would fetch all candidates from database");
    Ok(vec!["candidate1".to_string(), "candidate2".to_string()])
}

/// Get latest jobs from the database
async fn get_latest_jobs() -> Result<Vec<String>> {
    // For now, return mock job identifiers
    // In the actual implementation, this would use DatabaseProvider::get_latest_jobs()
    println!("Would fetch latest jobs from database");
    Ok(vec![
        "job1".to_string(),
        "job2".to_string(),
        "job3".to_string(),
    ])
}

/// Process candidate matches with rate limiting
async fn process_candidate_matches(
    candidates: Vec<String>,
    jobs: Vec<String>,
    config: &SchedulerConfig,
) -> Result<()> {
    let mut processed_count = 0;

    for (batch_idx, candidate_batch) in candidates
        .chunks(config.max_candidate_batch_size)
        .enumerate()
    {
        println!(
            "Processing batch {} ({} candidates)",
            batch_idx + 1,
            candidate_batch.len()
        );

        for candidate in candidate_batch {
            // Simulate rate limiting delay
            println!("Processing candidate: {}", candidate);
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            // Generate AI matches (simulated)
            println!(
                "  Generated {} matches for candidate {}",
                jobs.len(),
                candidate
            );

            // Store matches in database (simulated)
            println!("  Stored matches in database");

            processed_count += 1;
        }

        // Add delay between batches
        if batch_idx < candidates.len().saturating_sub(1) / config.max_candidate_batch_size {
            println!(
                "Waiting {} seconds before next batch...",
                config.batch_delay_seconds
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(config.batch_delay_seconds)).await;
        }
    }

    println!("Processed {} candidates total", processed_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_config_default() {
        let config = SchedulerConfig::default();
        assert_eq!(config.cron_schedule, "0 2 * * *");
        assert!(!config.debug);
        assert_eq!(config.max_candidate_batch_size, 5);
        assert_eq!(config.batch_delay_seconds, 30);
    }

    #[tokio::test]
    async fn test_scheduler_creation() -> Result<()> {
        let config = SchedulerConfig::default();
        let _scheduler = NightlyScheduler::new(config).await?;
        println!("Scheduler created successfully");
        Ok(())
    }

    #[tokio::test]
    async fn test_manual_trigger() -> Result<()> {
        let config = SchedulerConfig {
            cron_schedule: "0/1 * * * * *".to_string(), // Every second for testing
            debug: true,
            max_candidate_batch_size: 2,
            batch_delay_seconds: 1,
        };

        let scheduler = NightlyScheduler::new(config.clone()).await?;
        scheduler.trigger_manual().await?;
        println!("Manual trigger test completed");
        Ok(())
    }
}
