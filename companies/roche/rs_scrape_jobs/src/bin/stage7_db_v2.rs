use chrono::Utc;
use rs_scrape::{db_repo, db_setup, db_traits::DatabaseProvider, models::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = db_setup::init_db("jobs_minutils.db").await?;
    let repo = db_repo::JobRepository::new(conn);

    // Dummy candidate
    let candidate = Candidate {
        id: None,
        oauth_sub: "test_sub".to_string(),
        name: "Test User".to_string(),
        profile_text: "Some profile text".to_string(),
    };

    let candidate_id = repo.upsert_candidate(&candidate).await?;
    println!("Upserted candidate with id: {}", candidate_id);

    // Dummy match 1
    let match_data = CandidateMatch {
        id: None,
        candidate_id,
        job_identifier: "job123".to_string(),
        model_used: "gemini".to_string(),
        score: 0.85,
        explanation: "Good match".to_string(),
        created_at: Utc::now(),
    };

    repo.insert_candidate_match(&match_data).await?;

    // Dummy match 2
    let match_data2 = CandidateMatch {
        id: None,
        candidate_id,
        job_identifier: "job456".to_string(),
        model_used: "gemini".to_string(),
        score: 0.9,
        explanation: "Better match".to_string(),
        created_at: Utc::now(),
    };

    repo.insert_candidate_match(&match_data2).await?;

    // Get all matches for candidate, ordered by created_at DESC
    let matches: Vec<CandidateMatch> = repo.get_matches_for_candidate(candidate_id).await?;
    println!("All matches: {:?}", matches);

    // The latest one is the first in the list (since ORDER BY DESC)
    if let Some(latest) = matches.first() {
        println!("Latest match: {:?}", latest);
    }

    Ok(())
}
