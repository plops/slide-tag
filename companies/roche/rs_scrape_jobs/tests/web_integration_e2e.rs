use rs_scrape::{
    app_state::AppState, ai_core::AiProvider, config::AppConfig, db_repo::JobRepository, 
    db_setup, models::{Job, JobAnnotation, CandidateMatch}, web_server, db_traits::DatabaseProvider
};
use chromiumoxide_autowait::PageAutoWaitExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;

// MockAiProvider for testing without real AI calls
struct MockAiProvider;

#[async_trait]
impl AiProvider for MockAiProvider {
    async fn annotate_jobs(&self, _jobs: Vec<Job>) -> Result<Vec<JobAnnotation>> {
        // Return empty annotations for testing
        Ok(vec![])
    }

    async fn match_candidate(&self, _profile: &str, jobs: Vec<Job>) -> Result<Vec<CandidateMatch>> {
        // Return dummy matches for testing
        let mut matches = Vec::new();
        for job in jobs.into_iter() {
            matches.push(CandidateMatch {
                id: None,
                candidate_id: 1, // Dummy candidate ID
                job_identifier: job.identifier,
                model_used: "mock_model".to_string(),
                score: 0.8, // Dummy score
                explanation: "This is a mock match for testing purposes".to_string(),
                created_at: Utc::now(),
            });
        }
        Ok(matches)
    }
}

#[tokio::test]
async fn test_full_user_journey() -> Result<()> {
    // Set up environment variables for testing
    std::env::set_var("GITHUB_CLIENT_ID", "test_client_id");
    std::env::set_var("GITHUB_CLIENT_SECRET", "test_client_secret");
    std::env::set_var("OAUTH_REDIRECT_URL", "http://localhost:3040/auth/callback");
    std::env::set_var("DATABASE_URL", "test_e2e.db");

    // 1. Setup DB & AppState mit MockAiProvider
    let db_path = "test_e2e.db";
    
    // Clean up any existing test database
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path)?;
    }
    
    let conn = db_setup::init_db(db_path).await?;
    let repo = Arc::new(JobRepository::new(conn));
    
    // Insert 2 dummy jobs for testing
    let job1 = Job {
        identifier: "test_job_1".to_string(),
        title: "Senior Rust Engineer".to_string(),
        description: Some("A great Rust job opportunity".to_string()),
        location: "Basel".to_string(),
        organization: Some("Roche".to_string()),
        required_topics: Some(vec!["Rust".to_string(), "Backend".to_string()]),
        nice_to_haves: Some(vec!["Docker".to_string()]),
        pay_grade: Some("Senior".to_string()),
        sub_category: Some("Engineering".to_string()),
        category_raw: Some("Software".to_string()),
        employment_type: Some("Full-time".to_string()),
        work_hours: Some("40h".to_string()),
        worker_type: Some("Employee".to_string()),
        job_profile: Some("Senior".to_string()),
        supervisory_organization: Some("IT".to_string()),
        target_hire_date: Some("ASAP".to_string()),
        no_of_available_openings: Some("1".to_string()),
        grade_profile: Some("Senior".to_string()),
        recruiting_start_date: Some("2024-01-01".to_string()),
        job_level: Some("Senior".to_string()),
        job_family: Some("Engineering".to_string()),
        job_type: Some("Permanent".to_string()),
        is_evergreen: Some("false".to_string()),
        standardised_country: Some("Switzerland".to_string()),
        run_date: Some("2024-01-01".to_string()),
        run_id: Some("run_123".to_string()),
        address_locality: Some("Basel".to_string()),
        address_region: Some("Basel-Stadt".to_string()),
        address_country: Some("CH".to_string()),
        postal_code: Some("4058".to_string()),
        job_summary: Some("Great opportunity for Rust developers".to_string()),
    };

    let job2 = Job {
        identifier: "test_job_2".to_string(),
        title: "Full Stack Developer".to_string(),
        description: Some("Frontend and backend development".to_string()),
        location: "Zurich".to_string(),
        organization: Some("Roche".to_string()),
        required_topics: Some(vec!["JavaScript".to_string(), "React".to_string()]),
        nice_to_haves: Some(vec!["TypeScript".to_string()]),
        pay_grade: Some("Mid".to_string()),
        sub_category: Some("Engineering".to_string()),
        category_raw: Some("Software".to_string()),
        employment_type: Some("Full-time".to_string()),
        work_hours: Some("40h".to_string()),
        worker_type: Some("Employee".to_string()),
        job_profile: Some("Mid".to_string()),
        supervisory_organization: Some("IT".to_string()),
        target_hire_date: Some("ASAP".to_string()),
        no_of_available_openings: Some("1".to_string()),
        grade_profile: Some("Mid".to_string()),
        recruiting_start_date: Some("2024-01-01".to_string()),
        job_level: Some("Mid".to_string()),
        job_family: Some("Engineering".to_string()),
        job_type: Some("Permanent".to_string()),
        is_evergreen: Some("false".to_string()),
        standardised_country: Some("Switzerland".to_string()),
        run_date: Some("2024-01-01".to_string()),
        run_id: Some("run_123".to_string()),
        address_locality: Some("Zurich".to_string()),
        address_region: Some("Zurich".to_string()),
        address_country: Some("CH".to_string()),
        postal_code: Some("8001".to_string()),
        job_summary: Some("Great opportunity for web developers".to_string()),
    };

    repo.insert_job_history(&job1).await?;
    repo.insert_job_history(&job2).await?;

    // Create AppState with MockAiProvider
    let config = AppConfig {
        is_debug: true,
        github_client_id: "test_client_id".to_string(),
        github_client_secret: "test_client_secret".to_string(),
        oauth_redirect_url: "http://localhost:3040/auth/callback".to_string(),
        db_path: db_path.to_string(),
        session_max_age_days: 1,
        session_secure: false, // HTTP for testing
        gemini_api_key: "test_key".to_string(), // Dummy key for testing
        host: "127.0.0.1".to_string(),
        port: 3040,
        admin_username: "dev test user".to_string(),
    };

    let app_state = Arc::new(AppState {
        db: repo.clone() as Arc<dyn rs_scrape::db_traits::DatabaseProvider>,
        ai: Arc::new(MockAiProvider) as Arc<dyn AiProvider>,
        config: Arc::new(config),
        scrape_status: Arc::new(tokio::sync::RwLock::new(rs_scrape::app_state::ScrapeStatus::Idle)),
    });

    // 2. Server im Hintergrund starten (Port 3040)
    let port = 3040;
    let addr = format!("127.0.0.1:{}", port).parse()?;
    
    let server_app_state = app_state.clone();
    let server_config = server_app_state.config.clone();
    let server_handle = tokio::spawn(async move {
        if let Err(e) = web_server::run_server(addr, server_app_state, &server_config).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Wait for server to start
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // 3. Browser starten
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder().build().unwrap()
    ).await?;
    
    let handler_task = tokio::spawn(async move {
        while (handler.next().await).is_some() {}
    });
    
    let page = browser.new_page("about:blank").await?;

    // SCHRITT A: Ohne Login Jobs ansehen
    page.goto(format!("http://localhost:{}/jobs", port)).await?;
    
    // Auto-wait for the job card to appear! Kein manuelles Sleep mehr!
    page.auto_wait_visible(".job-card").await.expect("Job card did not appear");

    // SCHRITT B: Dev-Login
    page.goto(format!("http://localhost:{}/auth/dev-login", port)).await?;
    // Wir sollten direkt aufs Dashboard redirected werden. Wir warten bis die Stats laden:
    page.auto_wait_visible(".stat-card").await.expect("Dashboard failed to load");

    // SCHRITT C: Profil bearbeiten
    page.goto(format!("http://localhost:{}/profile", port)).await?;
    // Nutze auto_fill (wartet auf Visible, Enabled, Editable)
    page.auto_fill("#profile_text", "Senior Rust Engineer with AI experience").await?;
    // Nutze auto_click (wartet auf Visible, Enabled, Stable)
    page.auto_click("button[type=\"submit\"]").await?;

    // SCHRITT D: Match triggern
    page.goto(format!("http://localhost:{}/dashboard", port)).await?;
    
    // Auto-click auf den Re-evaluate Button
    page.auto_click("form[action=\"/api/trigger-match\"] button").await?;
    
    // Wait for potential navigation/redirect after trigger
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    
    // Nach kurzem Reload (da Background Job):
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    page.goto(format!("http://localhost:{}/dashboard", port)).await?;
    
    // Verify matches were created (using simple check instead of auto_wait_visible)
    let match_count = page.evaluate("document.querySelectorAll('.match-card').length".to_string()).await?;
    if match_count.into_value::<i32>().unwrap_or(0) == 0 {
        panic!("No matches were generated after trigger");
    }

    // SCHRITT E: Admin Funktion testen
    page.goto(format!("http://localhost:{}/admin", port)).await?;
    // Warten bis Admin Button bereit ist, dann klicken
    page.auto_click("form[action=\"/admin/trigger\"] button").await?;

    // Cleanup
    browser.close().await?;
    handler_task.await?;
    server_handle.abort();

    // Clean up test database
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path)?;
    }

    Ok(())
}
