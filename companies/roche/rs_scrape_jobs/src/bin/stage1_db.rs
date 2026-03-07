use rs_scrape::{db_repo::JobRepository, db_setup::init_db, models::Job};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = init_db("jobs_minutils.db").await?;
    let repo = JobRepository::new(conn);

    // Insert dummy jobs
    let job1 = Job {
        id: None,
        title: "Software Engineer".to_string(),
        description: Some("Develop software.".to_string()),
        location: "Remote".to_string(),
    };
    repo.insert_job(&job1).await?;

    let job2 = Job {
        id: None,
        title: "Data Scientist".to_string(),
        description: None,
        location: "Zurich".to_string(),
    };
    repo.insert_job(&job2).await?;

    // Get all jobs
    let jobs = repo.get_all_jobs().await?;
    for job in jobs {
        println!("{:?}", job);
    }

    Ok(())
}
