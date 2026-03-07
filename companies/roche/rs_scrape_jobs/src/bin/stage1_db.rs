use rs_scrape::{
    db_repo::JobRepository,
    db_setup::init_db,
    models::{Job, Location, Skill},
};

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

    // Insert dummy skills
    let skill1 = Skill {
        id: None,
        name: "Rust".to_string(),
    };
    repo.insert_skill(&skill1).await?;

    let skill2 = Skill {
        id: None,
        name: "Python".to_string(),
    };
    repo.insert_skill(&skill2).await?;

    // Insert dummy locations
    let loc1 = Location {
        id: None,
        name: "Remote".to_string(),
    };
    repo.insert_location(&loc1).await?;

    let loc2 = Location {
        id: None,
        name: "Zurich".to_string(),
    };
    repo.insert_location(&loc2).await?;

    // Associate jobs with skills
    repo.insert_job_skill(1, 1).await?; // job1 with Rust
    repo.insert_job_skill(2, 2).await?; // job2 with Python

    // Get all jobs
    let jobs = repo.get_all_jobs().await?;
    for job in jobs {
        println!("{:?}", job);
    }

    // Get all skills
    let skills = repo.get_all_skills().await?;
    for skill in skills {
        println!("{:?}", skill);
    }

    // Get all locations
    let locations = repo.get_all_locations().await?;
    for loc in locations {
        println!("{:?}", loc);
    }

    Ok(())
}
