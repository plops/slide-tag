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

// MockAiProvider für Testing ohne echte AI-Aufrufe
struct MockAiProvider;

#[async_trait]
impl AiProvider for MockAiProvider {
    async fn annotate_jobs(&self, _jobs: Vec<Job>) -> Result<Vec<JobAnnotation>> {
        // Leere Annotationen für Testing
        Ok(vec![])
    }

    async fn match_candidate(&self, _profile: &str, jobs: Vec<Job>) -> Result<Vec<CandidateMatch>> {
        // Dummy-Matches für Testing
        let mut matches = Vec::new();
        for job in jobs.into_iter() {
            matches.push(CandidateMatch {
                id: None,
                candidate_id: 1, // Dummy-Kandidaten-ID
                job_identifier: job.identifier,
                model_used: "mock_model".to_string(),
                score: 0.8, // Dummy-Score
                explanation: "This is a mock match for testing purposes".to_string(),
                created_at: Utc::now(),
            });
        }
        Ok(matches)
    }
}

#[tokio::test]
async fn test_ux_match_reload_problem() -> Result<()> {
    // Umgebung für Testing einrichten
    std::env::set_var("GITHUB_CLIENT_ID", "test_client_id");
    std::env::set_var("GITHUB_CLIENT_SECRET", "test_client_secret");
    std::env::set_var("OAUTH_REDIRECT_URL", "http://localhost:3040/auth/callback");
    std::env::set_var("DATABASE_URL", "test_ux_problem.db");

    // 1. Datenbank mit Test-Jobs einrichten
    let db_path = "test_ux_problem.db";
    
    // Alte Test-Datenbank löschen
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path)?;
    }
    
    let conn = db_setup::init_db(db_path).await?;
    let repo = Arc::new(JobRepository::new(conn));

    // Test-Jobs einfügen (mehrere Jobs für besseres Testing)
    let job1 = Job {
        identifier: "ux_test_job_1".to_string(),
        title: "UX Test Job 1".to_string(),
        description: Some("Ein Job für UX-Testing".to_string()),
        location: "Basel".to_string(),
        organization: Some("Roche".to_string()),
        required_topics: Some(vec!["UX".to_string(), "Testing".to_string()]),
        nice_to_haves: Some(vec!["Frontend".to_string()]),
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
        run_id: Some("ux_test".to_string()),
        address_locality: Some("Basel".to_string()),
        address_region: Some("Basel-Stadt".to_string()),
        address_country: Some("CH".to_string()),
        postal_code: Some("4058".to_string()),
        job_summary: Some("UX Testing Job 1".to_string()),
    };

    let job2 = Job {
        identifier: "ux_test_job_2".to_string(),
        title: "UX Test Job 2".to_string(),
        description: Some("Zweiter Job für besseres UX-Testing".to_string()),
        location: "Zurich".to_string(),
        organization: Some("Roche".to_string()),
        required_topics: Some(vec!["Backend".to_string(), "Rust".to_string()]),
        nice_to_haves: Some(vec!["Database".to_string()]),
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
        run_id: Some("ux_test".to_string()),
        address_locality: Some("Zurich".to_string()),
        address_region: Some("Zurich".to_string()),
        address_country: Some("CH".to_string()),
        postal_code: Some("8001".to_string()),
        job_summary: Some("UX Testing Job 2".to_string()),
    };

    repo.insert_job_history(&job1).await?;
    repo.insert_job_history(&job2).await?;

    // 2. AppState mit MockAiProvider erstellen
    let config = AppConfig {
        is_debug: true,
        github_client_id: "test_client_id".to_string(),
        github_client_secret: "test_client_secret".to_string(),
        oauth_redirect_url: "http://localhost:3040/auth/callback".to_string(),
        db_path: db_path.to_string(),
        session_max_age_days: 1,
        session_secure: false, // HTTP für Testing
        gemini_api_key: "test_key".to_string(), // Dummy-Key für Testing
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

    // 3. Server im Hintergrund starten
    let port = 3040;
    let addr = format!("127.0.0.1:{}", port).parse()?;
    
    let server_app_state = app_state.clone();
    let server_config = server_app_state.config.clone();
    let server_handle = tokio::spawn(async move {
        if let Err(e) = web_server::run_server(addr, server_app_state, &server_config).await {
            eprintln!("Server error: {}", e);
        }
    });

    // Kurz warten, damit Server startet
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // 4. Browser starten
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder().build().unwrap()
    ).await?;
    
    let handler_task = tokio::spawn(async move {
        while (handler.next().await).is_some() {}
    });
    
    let page = browser.new_page("about:blank").await?;

    // 5. Dev-Login durchführen
    page.goto(format!("http://localhost:{}/auth/dev-login", port)).await?;
    page.auto_wait_visible(".stat-card").await.expect("Dashboard failed to load");

    // 6. Profil einrichten mit Test-Text
    page.goto(format!("http://localhost:{}/profile", port)).await?;
    page.auto_fill("#profile_text", "UX Testing Profile with special keywords").await?;
    page.auto_click("button[type=\"submit\"]").await?;

    // 7. Dashboard besuchen und ANZAHL DER EXISTIERENDEN MATCHES prüfen
    page.goto(format!("http://localhost:{}/dashboard", port)).await?;
    
    // WICHTIG: Anzahl der Matches VOR dem Trigger prüfen
    let matches_before = page.evaluate("document.querySelectorAll('.match-card').length".to_string()).await?;
    println!("Matches VOR Trigger: {:?}", matches_before);
    
    if matches_before.into_value::<i32>().unwrap_or(0) > 0 {
        println!("WARNUNG: Es gibt bereits Matches vor dem Trigger - Test nicht gültig");
        return Ok(());
    }

    // 8. Match-Trigger klicken
    page.auto_click("form[action=\"/api/trigger-match\"] button").await?;
    
    // 9. WICHTIG: Auf Matches warten, aber mit Timeout und ohne manuellen Reload
    println!("Warte auf Matches nach Trigger (ohne manuellen Reload)...");
    
    let mut attempts = 0;
    let max_attempts = 10; // 10 Sekunden warten
    
    while attempts < max_attempts {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        
        let matches_after = page.evaluate("document.querySelectorAll('.match-card').length".to_string()).await?;
        let match_count = matches_after.into_value::<i32>().unwrap_or(0);
        
        println!("Versuch {}: Matches gefunden = {}", attempts + 1, match_count);
        
        if match_count > 0 {
            println!("✅ ERFOLG: Matches wurden nach {} Sekunden automatisch sichtbar", attempts + 1);
            break;
        }
        
        if match_count >= 2 { // Erwarte 2 Matches (für 2 Jobs)
            println!("✅ ERFOLG: Alle {} Matches wurden nach {} Sekunden automatisch sichtbar", match_count, attempts + 1);
            break;
        }
        
        attempts += 1;
    }
    
    if attempts >= max_attempts {
        println!("❌ FEHLER: Keine Matches nach {} Sekunden sichtbar - UX-Problem bestätigt", max_attempts);
        
        // Zusätzliche Diagnose: Seiteninhalt prüfen
        let page_content = page.evaluate("document.body.innerText".to_string()).await?;
        println!("Seiteninhalt: {:?}", page_content);
        
        return Err(anyhow::anyhow!("UX-Problem: Matches werden nicht automatisch angezeigt"));
    }

    // Cleanup
    browser.close().await?;
    handler_task.await?;
    server_handle.abort();
    
    // Test-Datenbank aufräumen
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path)?;
    }

    Ok(())
}
