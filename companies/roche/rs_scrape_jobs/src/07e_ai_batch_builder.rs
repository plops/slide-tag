use crate::ai_rate_limiter::SharedRateLimiter;
use crate::models::Job;

/// Builds batches of jobs to respect token limits
pub struct BatchBuilder {
    rate_limiter: SharedRateLimiter,
    jobs: Vec<Job>,
    current_tokens: u32,
    token_threshold: u32,
    assumed_words_per_token: f32,
}

impl BatchBuilder {
    pub async fn new(rate_limiter: SharedRateLimiter) -> Self {
        // Acquire the limiter to read config values for thresholds, then drop the guard
        let (token_threshold, assumed_words_per_token) = {
            let guard = rate_limiter.lock().await;
            (
                (guard.config.tpm_limit as f32 * 0.8) as u32,
                guard.config.assumed_words_per_token,
            )
        };

        Self {
            rate_limiter,
            jobs: Vec::new(),
            current_tokens: 0,
            token_threshold,
            assumed_words_per_token,
        }
    }

    /// Try to add a job to the current batch
    /// Returns true if added, false if it would exceed limit
    pub fn try_add_job(&mut self, job: Job) -> bool {
        let job_tokens = self.estimate_job_tokens(&job);

        if self.current_tokens + job_tokens > self.token_threshold {
            return false;
        }

        self.jobs.push(job);
        self.current_tokens += job_tokens;
        true
    }

    /// Get the current batch if it's not empty
    pub fn take_batch(&mut self) -> Option<Vec<Job>> {
        if self.jobs.is_empty() {
            return None;
        }

        let batch = std::mem::take(&mut self.jobs);
        self.current_tokens = 0;
        Some(batch)
    }

    /// Estimate tokens for a single job based on its text content
    pub fn estimate_job_tokens(&self, job: &Job) -> u32 {
        let mut total_words = 0;

        // Count words in title
        total_words += job.title.split_whitespace().count();

        // Count words in description
        if let Some(desc) = &job.description {
            total_words += desc.split_whitespace().count();
        }

        // Count words in location, organization, etc.
        total_words += job.location.split_whitespace().count();
        if let Some(org) = &job.organization {
            total_words += org.split_whitespace().count();
        }

        // Count words in required_topics and nice_to_haves
        if let Some(topics) = &job.required_topics {
            for topic in topics {
                total_words += topic.split_whitespace().count();
            }
        }
        if let Some(nice) = &job.nice_to_haves {
            for nice in nice {
                total_words += nice.split_whitespace().count();
            }
        }

        // Estimate tokens using stored assumption to avoid locking
        ((total_words as f32) / self.assumed_words_per_token) as u32
    }
}

/// Process jobs in batches respecting rate limits
pub async fn process_jobs_in_batches<F, Fut>(
    jobs: Vec<Job>,
    rate_limiter: SharedRateLimiter,
    mut processor: F,
) -> anyhow::Result<()>
where
    F: FnMut(Vec<Job>) -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    let mut builder = BatchBuilder::new(rate_limiter.clone()).await;
    let mut remaining_jobs = jobs;

    while !remaining_jobs.is_empty() {
        // Try to add jobs to batch until we can't add more
        while let Some(job) = remaining_jobs.pop() {
            if !builder.try_add_job(job.clone()) {
                // Job doesn't fit, put it back
                remaining_jobs.push(job);
                break;
            }
        }

        // Process the current batch
        if let Some(batch) = builder.take_batch() {
            // Wait for rate limiter to allow the request
            {
                let mut limiter = rate_limiter.lock().await;
                limiter.wait_for_request(builder.current_tokens).await;
                // Note: current_tokens is 0 after take_batch, but we need to estimate for the batch
                let batch_tokens = batch
                    .iter()
                    .map(|j| builder.estimate_job_tokens(j))
                    .sum::<u32>();
                limiter.record_request(batch_tokens);
            }

            // Process the batch
            processor(batch).await?;
        } else {
            // No more jobs to process
            break;
        }
    }

    Ok(())
}
