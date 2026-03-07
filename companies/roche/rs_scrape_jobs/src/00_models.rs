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
