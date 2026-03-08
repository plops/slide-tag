use rs_scrape::{
    ai_core::AiProvider, ai_gemini::GeminiProvider, db_repo::JobRepository, db_setup::init_db,
    models::JobAnnotation,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting stage 6: AI batch annotation test");

    let conn = init_db("jobs_minutils.db").await?;
    let repo = JobRepository::new(conn);

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

    // Load up to 20 unannotated jobs
    let jobs = repo.get_unannotated_jobs(20).await?;
    println!("Loaded {} unannotated jobs", jobs.len());

    if jobs.is_empty() {
        println!("No unannotated jobs found. Exiting.");
        return Ok(());
    }

    // Batch annotate jobs
    let annotations: Vec<JobAnnotation> = ai.annotate_jobs(jobs.clone()).await?;
    println!("Received annotations for {} jobs", annotations.len());

    // Update database with annotations
    for (job, annotation) in jobs.iter().zip(annotations.iter()) {
        let summary = annotation.job_summary.join("\n");
        let relevance = annotation.slide_tag_relevance.to_string();
        repo.update_job_ai(&job.identifier, &summary, &relevance)
            .await?;
        println!("Updated job {}", job.identifier);
    }

    println!("Stage 6 batch annotation test completed successfully!");
    Ok(())
}
