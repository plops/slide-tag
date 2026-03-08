use clap::Parser;
use rs_scrape::{db_repo, db_setup, pipeline_orchestrator};

#[derive(Parser)]
struct Args {
    /// Enable debug dump of HTML and JSON files to debug_dumps/YYYY-MM-DD/
    #[arg(long)]
    debug_dump: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let conn = db_setup::init_db("jobs_minutils.db").await?;
    let repo = db_repo::JobRepository::new(conn);

    pipeline_orchestrator::run_pipeline(&repo, args.debug_dump).await?;

    Ok(())
}
