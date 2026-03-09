use crate::models::{Candidate, CandidateMatch, Job};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseProvider: Send + Sync {
    async fn insert_job_history(&self, job: &Job) -> Result<()>;
    async fn get_latest_jobs(&self) -> Result<Vec<Job>>;
    async fn upsert_candidate(&self, candidate: &Candidate) -> Result<i64>;
    async fn insert_candidate_match(&self, match_data: &CandidateMatch) -> Result<()>;
    async fn get_matches_for_candidate(&self, candidate_id: i64) -> Result<Vec<CandidateMatch>>;
    async fn get_candidate_by_oauth_sub(&self, oauth_sub: &str) -> Result<Option<Candidate>>;
    async fn get_candidate_by_id(&self, candidate_id: i64) -> Result<Option<Candidate>>;
    async fn get_all_candidates(&self) -> Result<Vec<Candidate>>;
}
