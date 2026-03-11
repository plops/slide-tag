use crate::ai_batch_builder::BatchBuilder;
use crate::ai_core::AiProvider;
use crate::ai_rate_limiter::{create_rate_limiter, AiModelConfig, SharedRateLimiter};
use crate::models::{CandidateMatch, Job, JobAnnotation};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::CompletionModel;
use rig::providers::gemini;
use rig::telemetry::ProviderResponseExt;
use serde_json;

pub struct GeminiProvider {
    rate_limiter: SharedRateLimiter,
}

impl GeminiProvider {
    pub fn new(_api_key: &str) -> Result<Self> {
        // Gemini 3.1 flash lite preview limits
        let config = AiModelConfig {
            name: "gemini-3.1-flash-lite-preview".to_string(),
            rpm_limit: 15,
            tpm_limit: 250_000,
            rpd_limit: 500,
            assumed_words_per_token: 0.75, // 1 word ≈ 1.33 tokens, so 1/1.33 ≈ 0.75
            words_per_request: 25_000,     // 25,000 words per request limit
        };
        let rate_limiter = create_rate_limiter(config);

        Ok(Self { rate_limiter })
    }
}

#[async_trait]
impl AiProvider for GeminiProvider
where
    Self: Send + Sync,
{
    async fn annotate_jobs(&self, jobs: Vec<Job>) -> Result<Vec<JobAnnotation>> {
        let mut all_annotations = Vec::new();
        let mut batch_builder = BatchBuilder::new(self.rate_limiter.clone()).await;
        let mut remaining_jobs = jobs;
        
        // Progress tracking
        let total_jobs = remaining_jobs.len();
        let mut processed_jobs = 0;

        while !remaining_jobs.is_empty() {
            // Fill the batch
            while let Some(job) = remaining_jobs.pop() {
                if !batch_builder.try_add_job(job.clone()) {
                    // Job doesn't fit, put it back
                    remaining_jobs.push(job);
                    break;
                }
            }

            // Process the current batch
            if let Some(batch) = batch_builder.take_batch() {
                let batch_tokens = batch
                    .iter()
                    .map(|j| batch_builder.estimate_job_tokens(j))
                    .sum::<u32>();
                let batch_len = batch.len();

                // Log progress before processing
                println!("Processing job descriptions {}..{} out of {}", 
                    processed_jobs + 1, processed_jobs + batch_len, total_jobs);
                
                // Measure processing time
                let start_time = std::time::Instant::now();

                // Wait for rate limiter
                {
                    let mut limiter = self.rate_limiter.lock().await;
                    limiter.wait_for_request(batch_tokens).await;
                    limiter.record_request(batch_tokens);
                }

                // Process the batch
                let annotations = self.process_batch(batch).await?;
                all_annotations.extend(annotations);
                
                // Log completion and metrics
                let duration = start_time.elapsed();
                println!("Completed batch in {:?}, used {} tokens", duration, batch_tokens);
                
                processed_jobs += batch_len;
            } else {
                break;
            }
        }

        Ok(all_annotations)
    }

    #[allow(dead_code)]
    async fn match_candidate(&self, profile: &str, jobs: Vec<Job>) -> Result<Vec<CandidateMatch>> {
        if jobs.is_empty() {
            return Ok(vec![]);
        }

        let mut all_matches = Vec::new();
        let mut batch_builder = BatchBuilder::new(self.rate_limiter.clone()).await;
        let mut remaining_jobs = jobs;
        
        // Progress tracking
        let total_jobs = remaining_jobs.len();
        let mut processed_jobs = 0;

        // Process jobs in batches to respect rate limits
        while !remaining_jobs.is_empty() {
            // Fill the batch
            while let Some(job) = remaining_jobs.pop() {
                if !batch_builder.try_add_job(job.clone()) {
                    // Job doesn't fit, put it back
                    remaining_jobs.push(job);
                    break;
                }
            }

            // Process the current batch
            if let Some(batch) = batch_builder.take_batch() {
                let batch_tokens = batch
                    .iter()
                    .map(|j| batch_builder.estimate_job_tokens(j))
                    .sum::<u32>();
                let batch_len = batch.len();

                // Log progress before processing
                println!("Processing candidate matching for jobs {}..{} out of {}", 
                    processed_jobs + 1, processed_jobs + batch_len, total_jobs);
                
                // Measure processing time
                let start_time = std::time::Instant::now();

                // Wait for rate limiter
                {
                    let mut limiter = self.rate_limiter.lock().await;
                    limiter.wait_for_request(batch_tokens).await;
                    limiter.record_request(batch_tokens);
                }

                // Process the batch
                let matches = self
                    .process_candidate_matching_batch(profile, batch)
                    .await?;
                all_matches.extend(matches);
                
                // Log completion and metrics
                let duration = start_time.elapsed();
                println!("Completed candidate matching batch in {:?}, used {} tokens", duration, batch_tokens);
                
                processed_jobs += batch_len;
            } else {
                break;
            }
        }

        Ok(all_matches)
    }
}

impl GeminiProvider {
    /// Process a single batch of jobs and return annotations
    async fn process_batch(&self, jobs: Vec<Job>) -> Result<Vec<JobAnnotation>> {
        let client = gemini::Client::from_env();

        let mut input = String::new();

        for (i, job) in jobs.iter().enumerate() {
            input.push_str(&format!("### JOB_{} ###\n", i));
            input.push_str(&format!("Title: {}\n", job.title));
            input.push_str(&format!(
                "Description: {}\n",
                job.description.as_deref().unwrap_or("N/A")
            ));
            input.push_str(&format!("Location: {}\n", job.location));
            input.push_str(&format!(
                "Organization: {}\n",
                job.organization.as_deref().unwrap_or("N/A")
            ));
            input.push_str(&format!("Required Topics: {:?}\n", job.required_topics));
            input.push_str(&format!("Nice to Haves: {:?}\n", job.nice_to_haves));
            
            // Add ATS fields
            input.push_str(&format!("Job Level: {}\n", job.job_level.as_deref().unwrap_or("N/A")));
            input.push_str(&format!("Job Family: {}\n", job.job_family.as_deref().unwrap_or("N/A")));
            input.push_str(&format!("Grade Profile: {}\n", job.grade_profile.as_deref().unwrap_or("N/A")));
            input.push_str(&format!("Employment Type: {}\n", job.employment_type.as_deref().unwrap_or("N/A")));
            
            input.push('\n');
        }

        println!("Sending input to AI:\n{}", input);
        let completion_model = client.completion_model("gemini-3.1-flash-lite-preview");
        let preamble = "Analyze the job descriptions below in the context of Slide-tag and related spatial genomics technologies. These technologies integrate techniques like Next-Generation Sequencing (NGS), single-cell/nucleus RNA sequencing (sc/snRNA-seq), molecular pathology, and complex bioinformatics to map gene activity in tissue.

IMPORTANT: Respond in the ORIGINAL LANGUAGE of each job description (German for DE jobs, English for EN jobs). Do not translate.

The output should be a JSON array of objects, each with:
1. `job_summary`: A bullet-point summary (as an array of strings) of the key responsibilities and required qualifications.
2. `slide_tag_relevance`: An integer score from 1 (unrelated) to 5 (highly relevant), rating the job's connection to the development or application of these technologies.
3. `idx`: The index of the job in the input list (for tracking purposes).";

        let request = completion_model
            .completion_request(&input)
            .preamble(preamble.to_string())
            .build();

        // Retry logic - attempt up to 3 times
        let mut attempts = 0;
        let max_attempts = 3;
        
        loop {
            attempts += 1;
            match completion_model.completion(request.clone()).await {
                Ok(response) => {
                    let raw = response
                        .raw_response
                        .get_text_response()
                        .unwrap_or("No text response".to_string());
                    println!("AI raw response: {}", raw);

                    let cleaned = raw
                        .trim_start_matches("```json\n")
                        .trim_end_matches("\n```")
                        .trim();
                    let batch: Vec<JobAnnotation> = serde_json::from_str(cleaned)?;
                    println!("Parsed results: {:?}", batch);

                    // Validate that we have annotations for all jobs in the batch
                    let mut annotations = Vec::new();
                    for (i, _) in jobs.iter().enumerate() {
                        if let Some(annotation) = batch.iter().find(|a| a.idx == i as i32) {
                            annotations.push(annotation.clone());
                        } else {
                            println!(
                                "Missing annotation for job {} (ID: {})",
                                i, jobs[i].identifier
                            );
                            return Err(anyhow!("Missing annotation for job {}", i));
                        }
                    }

                    return Ok(annotations);
                }
                Err(e) => {
                    if attempts >= max_attempts {
                        return Err(anyhow!("Failed after {} attempts: {}", max_attempts, e));
                    }
                    println!("Warning: AI request failed (attempt {}/{}): {}. Retrying in 2 seconds...", 
                        attempts, max_attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    /// Process a single batch of jobs for candidate matching
    async fn process_candidate_matching_batch(
        &self,
        profile: &str,
        jobs: Vec<Job>,
    ) -> Result<Vec<CandidateMatch>> {
        let client = gemini::Client::from_env();

        let mut input = String::new();
        input.push_str(&format!("### CANDIDATE PROFILE ###\n{}\n\n", profile));

        input.push_str("### JOBS ###\n");
        for (i, job) in jobs.iter().enumerate() {
            input.push_str(&format!("### JOB_{} ###\n", i));
            input.push_str(&format!("Identifier: {}\n", job.identifier));
            input.push_str(&format!("Title: {}\n", job.title));
            input.push_str(&format!(
                "Job Summary: {}\n",
                job.job_summary.as_deref().unwrap_or("No summary available")
            ));
            input.push_str(&format!("Location: {}\n", job.location));
            input.push_str(&format!(
                "Organization: {}\n",
                job.organization.as_deref().unwrap_or("N/A")
            ));
            input.push_str(&format!("Required Topics: {:?}\n", job.required_topics));
            input.push_str(&format!("Nice to Haves: {:?}\n", job.nice_to_haves));
            input.push('\n');
        }

        println!("Sending candidate matching input to AI...");
        let completion_model = client.completion_model("gemini-3.1-flash-lite-preview");
        let preamble = "You are an expert recruiter matching candidates to job opportunities. 

Analyze the candidate profile against the provided JOB SUMMARIES (not full descriptions) and determine which jobs are the best matches.

The output should be a JSON array of objects, each with:
1. `job_identifier`: The identifier of the job (string)
2. `score`: A match score from 0.0 to 1.0 indicating how well the candidate matches the job
3. `explanation`: A brief explanation (max 200 characters) of why this job is a good or bad match
4. `idx`: The index of the job in the input list (for tracking purposes)

Only include jobs that have at least a 0.3 (30%) match score. Focus on relevant skills, experience, and qualifications.";

        let request = completion_model
            .completion_request(&input)
            .preamble(preamble.to_string())
            .build();

        // Retry logic - attempt up to 3 times
        let mut attempts = 0;
        let max_attempts = 3;
        
        loop {
            attempts += 1;
            match completion_model.completion(request.clone()).await {
                Ok(response) => {
                    let raw = response
                        .raw_response
                        .get_text_response()
                        .unwrap_or("No text response".to_string());
                    println!("AI matching raw response: {}", raw);

                    let cleaned = raw
                        .trim_start_matches("```json\n")
                        .trim_end_matches("\n```")
                        .trim();

                    // Parse the AI response
                    let batch: Vec<serde_json::Value> = serde_json::from_str(cleaned).map_err(|e| {
                        anyhow!(
                            "Failed to parse AI response as JSON: {}. Raw response: {}",
                            e,
                            raw
                        )
                    })?;

                    println!("Parsed matching results: {:?}", batch);

                    // Convert to CandidateMatch objects
                    let mut matches = Vec::new();
                    for item in batch {
                        if let (Some(job_identifier), Some(score), Some(explanation)) = (
                            item.get("job_identifier").and_then(|v| v.as_str()),
                            item.get("score").and_then(|v| v.as_f64()),
                            item.get("explanation").and_then(|v| v.as_str()),
                        ) {
                            matches.push(CandidateMatch {
                                id: None,
                                candidate_id: 0, // Will be set by caller
                                job_identifier: job_identifier.to_string(),
                                model_used: "gemini-3.1-flash-lite-preview".to_string(),
                                score: score as f32,
                                explanation: explanation.chars().take(200).collect::<String>(),
                                created_at: Utc::now(),
                            });
                        }
                    }

                    return Ok(matches);
                }
                Err(e) => {
                    if attempts >= max_attempts {
                        return Err(anyhow!("Failed after {} attempts: {}", max_attempts, e));
                    }
                    println!("Warning: AI matching request failed (attempt {}/{}): {}. Retrying in 2 seconds...", 
                        attempts, max_attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }
}
