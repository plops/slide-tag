use crate::ai_core::AiProvider;
use crate::models::{CandidateMatch, Job, JobAnnotation};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::CompletionModel;
use rig::providers::gemini;
use rig::telemetry::ProviderResponseExt;
use serde_json;

pub struct GeminiProvider {
    // model: gemini::CompletionModel,
}

impl GeminiProvider {
    pub fn new(_api_key: &str) -> Result<Self> {
        // let client = gemini::Client::new(api_key);
        // let model = client.model("gemini-3.1-flash-preview").build();
        // Ok(Self { model })
        Ok(Self {})
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn annotate_jobs(&self, jobs: Vec<Job>) -> Result<Vec<JobAnnotation>> {
        let client = gemini::Client::from_env();

        let mut input = String::new();

        for (i, job) in jobs.iter().enumerate() {
            input.push_str(&format!("### JOB_{} ###\n", i));
            input.push_str(&format!("Title: {}\n", job.title));
            input.push_str(&format!(
                "Description: {}\n",
                job.description
                    .as_deref()
                    .unwrap_or("N/A")
                    .chars()
                    .take(500)
                    .collect::<String>()
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

        println!("Sending input to AI:\n{}", input);
        let completion_model = client.completion_model("gemini-3.1-flash-lite-preview");
        let preamble = "Analyze the job descriptions below in the context of Slide-tag and related spatial genomics technologies. These technologies integrate techniques like Next-Generation Sequencing (NGS), single-cell/nucleus RNA sequencing (sc/snRNA-seq), molecular pathology, and complex bioinformatics to map gene activity in tissue.

The output should be a JSON array of objects, each with:
1. `job_summary`: A bullet-point summary (as an array of strings) of the key responsibilities and required qualifications.
2. `slide_tag_relevance`: An integer score from 1 (unrelated) to 5 (highly relevant), rating the job's connection to the development or application of these technologies.
3. `idx`: The index of the job in the input list (for tracking purposes).";

        let request = completion_model
            .completion_request(&input)
            .preamble(preamble.to_string())
            .build();
        let response = completion_model.completion(request).await?;
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

        Ok(annotations)
    }

    async fn match_candidate(
        &self,
        _profile: &str,
        _jobs: Vec<Job>,
    ) -> Result<Vec<CandidateMatch>> {
        // Stub implementation - to be implemented with AI matching logic
        Ok(vec![])
    }
}
