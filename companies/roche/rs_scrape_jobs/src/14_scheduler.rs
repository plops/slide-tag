use crate::app_state::AppState;
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
            cron_schedule: "0 0 2 * * *".to_string(), // 2:00 AM daily (sec min hour day month weekday)
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
    state: Arc<AppState>,
}

impl NightlyScheduler {
    /// Create a new scheduler instance
    pub async fn new(
        config: SchedulerConfig,
        state: Arc<AppState>,
    ) -> Result<Self, JobSchedulerError> {
        let scheduler = JobScheduler::new().await?;

        Ok(Self {
            scheduler: Arc::new(Mutex::new(scheduler)),
            config,
            state,
        })
    }

    /// Start the scheduler and begin executing jobs
    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        let config = self.config.clone();

        // Create the main nightly job
        let state = self.state.clone();
        let job = CronJob::new_async(config.cron_schedule.clone(), move |_uuid, _l| {
            let config = config.clone();
            let state = state.clone();

            Box::pin(async move {
                if let Err(e) = execute_nightly_pipeline(config, state).await {
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
        execute_nightly_pipeline(self.config.clone(), self.state.clone()).await
    }
}

/// Execute the complete nightly pipeline
async fn execute_nightly_pipeline(config: SchedulerConfig, state: Arc<AppState>) -> Result<()> {
    let start_time = Utc::now();
    println!("Starting nightly pipeline at {}", start_time);

    // Step 1: Scrape new jobs (placeholder - wird in Schritt 5 implementiert)
    println!("Step 1: Job scraping wird in Schritt 5 implementiert");
    // TODO: crate::pipeline_orchestrator::run_pipeline(&*state.db, config.debug).await?;

    // Step 2: Get all candidates for matching
    println!("Step 2: Fetching candidates for AI matching...");
    let candidates = state.db.get_all_candidates().await?;

    if candidates.is_empty() {
        println!("No candidates found, skipping AI matching");
        return Ok(());
    }

    println!("Found {} candidates to process", candidates.len());

    // Step 3: Get latest jobs for matching
    println!("Step 3: Fetching latest jobs for matching...");
    let jobs = state.db.get_latest_jobs().await?;

    if jobs.is_empty() {
        println!("No jobs found, skipping AI matching");
        return Ok(());
    }

    println!("Found {} jobs to match against", jobs.len());

    // Step 4: Process candidates in batches with rate limiting
    println!("Step 4: Processing AI matches with rate limiting...");
    process_candidate_matches(candidates, jobs, &config, &state).await?;

    let end_time = Utc::now();
    let duration = end_time - start_time;
    println!("Nightly pipeline completed successfully in {}", duration);

    Ok(())
}

/// Process candidate matches with rate limiting
async fn process_candidate_matches(
    candidates: Vec<crate::models::Candidate>,
    jobs: Vec<crate::models::Job>,
    config: &SchedulerConfig,
    state: &Arc<AppState>,
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
            println!("Processing candidate: {}", candidate.name);

            // Generate AI matches
            match state
                .ai
                .match_candidate(&candidate.profile_text, jobs.clone())
                .await
            {
                Ok(matches) => {
                    println!(
                        "  Generated {} matches for candidate {}",
                        matches.len(),
                        candidate.name
                    );

                    // Store matches in database
                    for match_data in matches {
                        let mut final_match = match_data;
                        final_match.candidate_id = candidate.id.unwrap_or(0);
                        if let Err(e) = state.db.insert_candidate_match(&final_match).await {
                            eprintln!("Failed to store match: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to generate matches for candidate {}: {}",
                        candidate.name, e
                    );
                }
            }

            processed_count += 1;
        }

        // Add delay between batches
        if batch_idx < (candidates.len().saturating_sub(1) / config.max_candidate_batch_size) {
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
        assert_eq!(config.cron_schedule, "0 0 2 * * *");
        assert!(!config.debug);
        assert_eq!(config.max_candidate_batch_size, 5);
        assert_eq!(config.batch_delay_seconds, 30);
    }

    #[tokio::test]
    async fn test_scheduler_creation() -> Result<()> {
        // Note: This test would need a mock AppState to work properly
        // For now, we'll skip this test until we have proper mocking
        println!("Scheduler creation test skipped - needs AppState mock");
        Ok(())
    }

    #[tokio::test]
    async fn test_manual_trigger() -> Result<()> {
        // Note: This test would need a mock AppState to work properly
        println!("Manual trigger test skipped - needs AppState mock");
        Ok(())
    }
}
