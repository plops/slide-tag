use chrono::{DateTime, Utc};
#[cfg(feature = "ai")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub organization: Option<String>,
    pub required_topics: Option<Vec<String>>,
    pub nice_to_haves: Option<Vec<String>>,
    pub pay_grade: Option<String>,
    pub sub_category: Option<String>,
    pub category_raw: Option<String>,
    pub employment_type: Option<String>,
    pub work_hours: Option<String>,
    pub worker_type: Option<String>,
    pub job_profile: Option<String>,
    pub supervisory_organization: Option<String>,
    pub target_hire_date: Option<String>,
    pub no_of_available_openings: Option<String>,
    pub grade_profile: Option<String>,
    pub recruiting_start_date: Option<String>,
    pub job_level: Option<String>,
    pub job_family: Option<String>,
    pub job_type: Option<String>,
    pub is_evergreen: Option<String>,
    pub standardised_country: Option<String>,
    pub run_date: Option<String>,
    pub run_id: Option<String>,
    pub address_locality: Option<String>,
    pub address_region: Option<String>,
    pub address_country: Option<String>,
    pub postal_code: Option<String>,
    pub job_summary: Option<String>,
    pub slide_tag_relevance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub id: Option<i64>,
    pub oauth_sub: String,
    pub name: String,
    pub profile_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobHistory {
    pub id: Option<i64>,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub organization: Option<String>,
    pub required_topics: Option<Vec<String>>,
    pub nice_to_haves: Option<Vec<String>>,
    pub pay_grade: Option<String>,
    pub sub_category: Option<String>,
    pub category_raw: Option<String>,
    pub employment_type: Option<String>,
    pub work_hours: Option<String>,
    pub worker_type: Option<String>,
    pub job_profile: Option<String>,
    pub supervisory_organization: Option<String>,
    pub target_hire_date: Option<String>,
    pub no_of_available_openings: Option<String>,
    pub grade_profile: Option<String>,
    pub recruiting_start_date: Option<String>,
    pub job_level: Option<String>,
    pub job_family: Option<String>,
    pub job_type: Option<String>,
    pub is_evergreen: Option<String>,
    pub standardised_country: Option<String>,
    pub run_date: Option<String>,
    pub run_id: Option<String>,
    pub address_locality: Option<String>,
    pub address_region: Option<String>,
    pub address_country: Option<String>,
    pub postal_code: Option<String>,
    pub job_summary: Option<String>,
    pub slide_tag_relevance: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: Option<i64>,
    pub name: String,
}

#[cfg(feature = "ai")]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct JobAnnotation {
    /// Bullet-point summary of key responsibilities and qualifications
    pub job_summary: Vec<String>,
    /// Relevance score from 1 (unrelated) to 5 (highly relevant)
    pub slide_tag_relevance: i32,
    /// Index of the job in the input list
    pub idx: i32,
}

#[cfg(feature = "ai")]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BatchAnnotationResult {
    /// List of job annotations
    pub results: Vec<JobAnnotation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandidateMatch {
    pub id: Option<i64>,
    pub candidate_id: i64,
    pub job_identifier: String,
    pub model_used: String,
    pub score: f32,
    pub explanation: String,
    pub created_at: DateTime<Utc>,
}
