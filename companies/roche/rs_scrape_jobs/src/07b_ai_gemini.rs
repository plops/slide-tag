use crate::ai_core::AiProvider;
use crate::models::{Job, JobAnnotation, BatchAnnotationResult, CandidateMatch};
use anyhow::Result;
use async_trait::async_trait;
use rig::client::{CompletionClient, ProviderClient};
use rig::providers::gemini;
use rig::providers::gemini::completion::GEMINI_2_5_FLASH;

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
        let extractor = client
            .extractor::<BatchAnnotationResult>(GEMINI_2_5_FLASH)
            .preamble("Analyze the following job postings and provide concise summaries and relevance assessments to slide_tag (a tool for creating presentations). For each job, give a brief summary and assess its relevance on a scale of 1-10 with explanation. Return results in JSON format with job IDs as keys.")
            .build();

        let mut input = String::new();
        let mut id_to_identifier = std::collections::HashMap::new();

        for (i, job) in jobs.iter().enumerate() {
            let id = i as u32;
            id_to_identifier.insert(id, job.identifier.clone());
            input.push_str(&format!("### JOB_{} ###\n", id));
            input.push_str(&format!("Title: {}\n", job.title));
            input.push_str(&format!("Description: {}\n", job.description.as_deref().unwrap_or("N/A")));
            input.push_str(&format!("Location: {}\n", job.location));
            input.push_str(&format!("Organization: {}\n", job.organization.as_deref().unwrap_or("N/A")));
            input.push_str(&format!("Required Topics: {:?}\n", job.required_topics));
            input.push_str(&format!("Nice to Haves: {:?}\n", job.nice_to_haves));
            input.push('\n');
        }

        let result: BatchAnnotationResult = extractor.extract(input).await?;
        let mut annotations: Vec<JobAnnotation> = Vec::new();

        for i in 0..jobs.len() {
            if let Some(annotation) = result.results.get(&(i as u32)) {
                annotations.push(annotation.clone());
            } else {
                return Err(anyhow::anyhow!("Missing annotation for job {}", i));
            }
        }

        Ok(annotations)
    }

    async fn match_candidate(&self, _profile: &str, _jobs: Vec<Job>) -> Result<Vec<CandidateMatch>> {
        // Stub implementation - to be implemented with AI matching logic
        Ok(vec![])
    }
}
