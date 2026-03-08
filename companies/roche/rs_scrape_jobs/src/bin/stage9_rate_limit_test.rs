use rs_scrape::{ai_core::AiProvider, ai_gemini::GeminiProvider, models::Job};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting stage 9: Rate limiting and batching test");

    let api_key = match std::env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            let home = std::env::var("HOME").map_err(|_| {
                anyhow::anyhow!("GEMINI_API_KEY not set and HOME env var not found")
            })?;
            let path = format!("{}/api_nano.txt", home);
            std::fs::read_to_string(&path)
                .map_err(|_| {
                    anyhow::anyhow!("GEMINI_API_KEY not set and ~/api_nano.txt not found")
                })?
                .trim()
                .to_string()
        }
    };
    std::env::set_var("GEMINI_API_KEY", &api_key);
    let ai = GeminiProvider::new(&api_key)?;

    // Create test jobs with varying content to test batching
    let jobs = vec![
        Job {
            identifier: "test-job-1".to_string(),
            title: "Senior Scientist Genomics".to_string(),
            description: Some("We are looking for a senior scientist with experience in spatial genomics and single-cell RNA sequencing. The role involves developing new technologies for mapping gene activity in tissue samples using advanced bioinformatics and molecular pathology techniques.".to_string()),
            location: "Basel, Switzerland".to_string(),
            organization: Some("Roche Diagnostics".to_string()),
            required_topics: Some(vec!["NGS".to_string(), "scRNA-seq".to_string(), "Bioinformatics".to_string()]),
            nice_to_haves: Some(vec!["Molecular Pathology".to_string(), "Spatial Transcriptomics".to_string()]),
            pay_grade: None,
            sub_category: None,
            category_raw: None,
            employment_type: None,
            work_hours: None,
            worker_type: None,
            job_profile: None,
            supervisory_organization: None,
            target_hire_date: None,
            no_of_available_openings: None,
            grade_profile: None,
            recruiting_start_date: None,
            job_level: None,
            job_family: None,
            job_type: None,
            is_evergreen: None,
            standardised_country: None,
            run_date: None,
            run_id: None,
            address_locality: None,
            address_region: None,
            address_country: None,
            postal_code: None,
            job_summary: None,
        },
        Job {
            identifier: "test-job-2".to_string(),
            title: "Bioinformatics Engineer".to_string(),
            description: Some("Join our team to work on computational pipelines for analyzing high-throughput sequencing data. Experience with Python, R, and cloud computing platforms required. Knowledge of machine learning for biological data analysis is a plus.".to_string()),
            location: "Basel, Switzerland".to_string(),
            organization: Some("Roche Pharmaceuticals".to_string()),
            required_topics: Some(vec!["Python".to_string(), "R".to_string(), "Cloud Computing".to_string()]),
            nice_to_haves: Some(vec!["Machine Learning".to_string(), "Hadoop".to_string()]),
            pay_grade: None,
            sub_category: None,
            category_raw: None,
            employment_type: None,
            work_hours: None,
            worker_type: None,
            job_profile: None,
            supervisory_organization: None,
            target_hire_date: None,
            no_of_available_openings: None,
            grade_profile: None,
            recruiting_start_date: None,
            job_level: None,
            job_family: None,
            job_type: None,
            is_evergreen: None,
            standardised_country: None,
            run_date: None,
            run_id: None,
            address_locality: None,
            address_region: None,
            address_country: None,
            postal_code: None,
            job_summary: None,
        },
        Job {
            identifier: "test-job-3".to_string(),
            title: "Research Associate Molecular Biology".to_string(),
            description: Some("Exciting opportunity to work on cutting-edge molecular biology projects. The position involves laboratory work with PCR, sequencing, and cell culture techniques. Experience with Roche diagnostic platforms is highly desirable.".to_string()),
            location: "Basel, Switzerland".to_string(),
            organization: Some("Roche Diagnostics".to_string()),
            required_topics: Some(vec!["PCR".to_string(), "Sequencing".to_string(), "Cell Culture".to_string()]),
            nice_to_haves: Some(vec!["Roche Platforms".to_string(), "Molecular Diagnostics".to_string()]),
            pay_grade: None,
            sub_category: None,
            category_raw: None,
            employment_type: None,
            work_hours: None,
            worker_type: None,
            job_profile: None,
            supervisory_organization: None,
            target_hire_date: None,
            no_of_available_openings: None,
            grade_profile: None,
            recruiting_start_date: None,
            job_level: None,
            job_family: None,
            job_type: None,
            is_evergreen: None,
            standardised_country: None,
            run_date: None,
            run_id: None,
            address_locality: None,
            address_region: None,
            address_country: None,
            postal_code: None,
            job_summary: None,
        },
    ];

    println!("Testing rate limiting with {} jobs", jobs.len());

    // This will now use rate limiting and batching
    let start_time = std::time::Instant::now();
    let annotations = ai.annotate_jobs(jobs).await?;
    let elapsed = start_time.elapsed();

    println!(
        "Received {} annotations in {:.2} seconds",
        annotations.len(),
        elapsed.as_secs_f32()
    );

    for annotation in &annotations {
        println!(
            "Job {}: summary points: {}",
            annotation.idx,
            annotation.job_summary.len()
        );
    }

    println!("Stage 9 rate limiting test completed successfully!");
    Ok(())
}
