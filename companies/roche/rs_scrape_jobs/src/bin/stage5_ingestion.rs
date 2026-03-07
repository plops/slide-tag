use rs_scrape::{
    data_ingestion::ingest_jobs_from_files, db_repo::JobRepository, db_setup::init_db,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = init_db("jobs_minutils.db").await?;
    let repo = JobRepository::new(conn);

    println!("Starting data ingestion from jobs_html/...");
    ingest_jobs_from_files(&repo).await?;
    println!("Data ingestion completed.");

    // Print all jobs to verify
    let jobs = repo.get_all_jobs().await?;
    println!("Total jobs in DB: {}", jobs.len());
    for job in jobs {
        println!(
            "Job: {} - {} (ID: {})",
            job.title, job.location, job.identifier
        );
    }

    Ok(())
}
