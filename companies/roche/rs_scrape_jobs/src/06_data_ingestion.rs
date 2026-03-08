use crate::db_repo::JobRepository;
use crate::models::Job;
use anyhow::Result;
use serde_json::Value;
use std::fs;

pub fn parse_roche_job(json_str: &str) -> Result<Job> {
    let value: Value = serde_json::from_str(json_str)?;
    let job = &value["jobDetail"]["data"]["job"];

    let title = job["ml_title"]
        .as_str()
        .unwrap_or("Unknown Title")
        .to_string();
    let description = job["structureData"]["description"]
        .as_str()
        .map(|s| s.to_string());
    let location = job["structureData"]["jobLocation"]["address"]["addressLocality"]
        .as_str()
        .unwrap_or("Unknown Location")
        .to_string();
    let identifier = job["structureData"]["identifier"]["value"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Identifier not found in JSON"))?
        .to_string();
    let organization = job["structureData"]["identifier"]["name"]
        .as_str()
        .map(|s: &str| s.to_string());
    let required_topics = job["ml_skills"].as_array().map(|arr| {
        arr.iter()
            .filter_map(|v| v.as_str())
            .map(|s: &str| s.to_string())
            .collect()
    });
    let nice_to_haves = None; // No nice_to_haves in JSON
    let pay_grade = job["additionalFields"]["grade"]
        .as_str()
        .map(|s: &str| s.to_string());
    let sub_category = job["subCategory"].as_str().map(|s: &str| s.to_string());
    let category_raw = job["category_raw"].as_str().map(|s: &str| s.to_string());
    let employment_type = job["structureData"]["employmentType"]
        .as_str()
        .map(|s: &str| s.to_string());
    let work_hours = job["structureData"]["workHours"]
        .as_str()
        .map(|s: &str| s.to_string());
    let worker_type = job["additionalFields"]["workerType"]
        .as_str()
        .map(|s: &str| s.to_string());
    let job_profile = job["additionalFields"]["jobProfile"]
        .as_str()
        .map(|s: &str| s.to_string());
    let supervisory_organization = job["additionalFields"]["supervisoryOrganization"]
        .as_str()
        .map(|s: &str| s.to_string());
    let target_hire_date = job["additionalFields"]["targetHireDate"]
        .as_str()
        .map(|s: &str| s.to_string());
    let no_of_available_openings = job["additionalFields"]["noOfAvailableOpenings"]
        .as_str()
        .map(|s: &str| s.to_string());
    let grade_profile = job["additionalFields"]["gradeProfile"]
        .as_str()
        .map(|s: &str| s.to_string());
    let recruiting_start_date = job["additionalFields"]["recruitingStartDate"]
        .as_str()
        .map(|s: &str| s.to_string());
    let job_level = job["additionalFields"]["jobLevel"]
        .as_str()
        .map(|s: &str| s.to_string());
    let job_family = job["additionalFields"]["jobFamily"]
        .as_str()
        .map(|s: &str| s.to_string());
    let job_type = job["additionalFields"]["jobType"]
        .as_str()
        .map(|s: &str| s.to_string());
    let is_evergreen = job["additionalFields"]["isEvergreen"]
        .as_str()
        .map(|s: &str| s.to_string());
    let standardised_country = job["standardisedCountry"]
        .as_str()
        .map(|s: &str| s.to_string());
    let run_date = job["metadata"]["runDate"]
        .as_str()
        .map(|s: &str| s.to_string());
    let run_id = job["metadata"]["runId"]
        .as_str()
        .map(|s: &str| s.to_string());
    let address_locality = job["structureData"]["jobLocation"]["address"]["addressLocality"]
        .as_str()
        .map(|s: &str| s.to_string());
    let address_region = job["structureData"]["jobLocation"]["address"]["addressRegion"]
        .as_str()
        .map(|s: &str| s.to_string());
    let address_country = job["structureData"]["jobLocation"]["address"]["addressCountry"]
        .as_str()
        .map(|s: &str| s.to_string());
    let postal_code = job["structureData"]["jobLocation"]["address"]["postalCode"]
        .as_str()
        .map(|s: &str| s.to_string());

    Ok(Job {
        identifier,
        title,
        description,
        location,
        organization,
        required_topics,
        nice_to_haves,
        pay_grade,
        sub_category,
        category_raw,
        employment_type,
        work_hours,
        worker_type,
        job_profile,
        supervisory_organization,
        target_hire_date,
        no_of_available_openings,
        grade_profile,
        recruiting_start_date,
        job_level,
        job_family,
        job_type,
        is_evergreen,
        standardised_country,
        run_date,
        run_id,
        address_locality,
        address_region,
        address_country,
        postal_code,
        job_summary: None,
    })
}

pub async fn ingest_jobs_from_files(repo: &JobRepository) -> Result<()> {
    let entries = fs::read_dir("jobs_html")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new("json")) {
            let content = fs::read_to_string(&path)?;
            match parse_roche_job(&content) {
                Ok(job) => {
                    repo.insert_job(&job).await?;
                    println!("Inserted job: {}", job.title);
                }
                Err(e) => {
                    eprintln!("Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }
    Ok(())
}
