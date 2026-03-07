use crate::models::{Job, JobAnnotation, CandidateMatch};
use anyhow::Result;
use async_trait::async_trait;

/// Trait for AI providers to handle job annotation tasks
#[async_trait]
pub trait AiProvider {
    async fn annotate_jobs(&self, jobs: Vec<Job>) -> Result<Vec<JobAnnotation>>;
    async fn match_candidate(&self, profile: &str, jobs: Vec<Job>) -> Result<Vec<CandidateMatch>>;
}
