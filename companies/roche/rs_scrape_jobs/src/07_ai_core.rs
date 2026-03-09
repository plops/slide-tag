use crate::models::{CandidateMatch, Job, JobAnnotation};
use anyhow::Result;
use async_trait::async_trait;

/// Trait for AI providers to handle job annotation tasks
#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn annotate_jobs(&self, jobs: Vec<Job>) -> Result<Vec<JobAnnotation>>;
    async fn match_candidate(&self, profile: &str, jobs: Vec<Job>) -> Result<Vec<CandidateMatch>>;
}
