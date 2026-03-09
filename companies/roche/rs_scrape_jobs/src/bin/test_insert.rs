use rs_scrape::{
    db_repo::JobRepository, db_setup::init_db, db_traits::DatabaseProvider, models::Job,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Testing database insertion...");

    // Create fresh database
    let conn = init_db("test_insert.db").await?;
    let repo = JobRepository::new(conn);

    // Create a simple test job
    let job = Job {
        identifier: "test_job_1".to_string(),
        title: "Test Engineer".to_string(),
        description: Some("Test description".to_string()),
        location: "Test Location".to_string(),
        organization: Some("Test Org".to_string()),
        required_topics: None,
        nice_to_haves: None,
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
    };

    println!("Inserting test job...");
    repo.insert_job_history(&job).await?;

    println!("✅ Insert successful!");

    // Test retrieval
    let jobs = repo.get_latest_jobs().await?;
    println!("Retrieved {} jobs", jobs.len());

    Ok(())
}
