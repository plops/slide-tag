use askama::Template;
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_sessions::Session;

use crate::{
    app_state::AppState, db_traits::DatabaseProvider, models::Candidate, models::CandidateMatch,
    models::JobHistory,
};

// Template structs for Askama

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub title: String,
    pub user_name: String,
    pub profile_text: String,
    pub success: bool,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub title: String,
    pub user_name: String,
    pub matches_count: usize,
    pub high_score_count: usize,
    pub good_fit_count: usize,
    pub has_matches: bool,
    pub matches_with_jobs: Vec<MatchWithJob>,
}

#[derive(Template)]
#[template(path = "match_detail.html")]
pub struct MatchDetailTemplate {
    pub title: String,
    pub user_name: String,
    pub match_data: CandidateMatch,
    pub job_title: String,
    pub job_location: String,
    pub job_description: Option<String>,
    pub job_organization: Option<String>,
    pub job_employment_type: Option<String>,
    pub job_level: Option<String>,
    pub job_family: Option<String>,
}

#[derive(Template)]
#[template(path = "jobs.html")]
pub struct JobsTemplate {
    pub title: String,
    pub user_name: String,
    pub jobs: Vec<JobHistory>,
    pub current_page: i64,
    pub total_pages: i64,
    pub search_query: String,
    pub total_count: i64,
    pub has_search: bool,
}

#[derive(Template)]
#[template(path = "job_detail.html")]
pub struct JobDetailTemplate {
    pub title: String,
    pub user_name: String,
    pub job: JobHistory,
}

// Data structure for dashboard - simplified for Askama
#[derive(Serialize)]
pub struct MatchWithJob {
    pub match_data: CandidateMatch,
    pub job: Option<JobHistory>,
    pub match_score_percent: i32,
    pub match_date_formatted: String,
}

// Form data structures
#[derive(Deserialize)]
pub struct ProfileForm {
    pub profile_text: String,
}

#[derive(Deserialize)]
pub struct JobQuery {
    pub page: Option<i64>,
    pub q: Option<String>,
}

// Error handling
#[derive(Debug)]
pub enum WebError {
    Database(anyhow::Error),
    Template(askama::Error),
    Auth(String),
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::Database(e) => write!(f, "Database error: {}", e),
            WebError::Template(e) => write!(f, "Template error: {}", e),
            WebError::Auth(msg) => write!(f, "Authentication error: {}", msg),
        }
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            WebError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error occurred".to_string(),
                )
            }
            WebError::Template(e) => {
                tracing::error!("Template error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Template rendering error".to_string(),
                )
            }
            WebError::Auth(msg) => {
                tracing::warn!("Authentication error: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head><title>Error</title></head>
<body>
    <h1>Error</h1>
    <p>{}</p>
    <a href="/">Go to Home</a>
</body>
</html>"#,
            error_message
        );

        (status, Html(html)).into_response()
    }
}

impl From<anyhow::Error> for WebError {
    fn from(err: anyhow::Error) -> Self {
        WebError::Database(err)
    }
}

impl From<askama::Error> for WebError {
    fn from(err: askama::Error) -> Self {
        WebError::Template(err)
    }
}

// Helper functions
async fn get_current_user(
    session: &Session,
    db_provider: &dyn DatabaseProvider,
) -> Result<Candidate, WebError> {
    let oauth_sub = session
        .get::<String>("oauth_sub")
        .await
        .map_err(|_| WebError::Auth("Failed to get oauth_sub from session".to_string()))?
        .ok_or_else(|| WebError::Auth("No oauth_sub in session".to_string()))?;

    // Debug: Log session data and oauth_sub
    tracing::info!("get_current_user: Session ID: {:?}, oauth_sub: {}", 
        session.id(), oauth_sub);

    db_provider
        .get_candidate_by_oauth_sub(&oauth_sub)
        .await
        .map_err(WebError::Database)?
        .ok_or_else(|| WebError::Auth("Candidate not found".to_string()))
}

// Route handlers
pub async fn get_profile(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Get current user
    let candidate = get_current_user(&session, &*state.db).await?;

    let success = session
        .get::<bool>("success")
        .await
        .unwrap_or(None)
        .unwrap_or(false);

    // Clear the success flag
    let _ = session.remove_value("success").await;

    let template = ProfileTemplate {
        title: "Profile".to_string(),
        user_name: candidate.name,
        profile_text: candidate.profile_text,
        success,
    };

    Ok(Html(template.render()?))
}

pub async fn post_profile(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<ProfileForm>,
) -> Result<Redirect, WebError> {
    // Get current user
    let candidate = get_current_user(&session, &*state.db).await?;

    // Update candidate profile in database
    let updated_candidate = Candidate {
        id: candidate.id,
        oauth_sub: candidate.oauth_sub,
        name: candidate.name,
        profile_text: form_data.profile_text.clone(),
    };

    state
        .db
        .upsert_candidate(&updated_candidate)
        .await
        .map_err(WebError::Database)?;

    // Set success flag in session
    session
        .insert("success", true)
        .await
        .map_err(|_| WebError::Auth("Failed to set success flag".to_string()))?;

    tracing::info!("Profile updated for user: {}", updated_candidate.oauth_sub);

    // Redirect to dashboard to show updated matches
    Ok(Redirect::to("/dashboard"))
}

pub async fn get_dashboard(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Get current user
    let candidate = get_current_user(&session, &*state.db).await?;
    
    // Debug: Log candidate info
    tracing::info!("Dashboard: Retrieved candidate - ID: {}, Name: {}, OAuth: {}", 
        candidate.id.unwrap_or(0), 
        candidate.name, 
        candidate.oauth_sub
    );

    // Get matches for this candidate
    let matches = state
        .db
        .get_matches_for_candidate(candidate.id.unwrap_or(0))
        .await
        .map_err(WebError::Database)?;
    
    // Debug: Log match count
    tracing::info!("Dashboard: Retrieved {} matches for candidate ID: {}", 
        matches.len(), 
        candidate.id.unwrap_or(0)
    );

    // Get latest jobs for mapping
    let jobs = state
        .db
        .get_latest_jobs()
        .await
        .map_err(WebError::Database)?;

    // Create job lookup map
    let mut job_map = std::collections::HashMap::new();
    for job in jobs {
        let job_history = JobHistory {
            id: None,
            identifier: job.identifier.clone(),
            title: job.title.clone(),
            description: job.description.clone(),
            location: job.location.clone(),
            organization: job.organization.clone(),
            required_topics: job.required_topics.clone(),
            nice_to_haves: job.nice_to_haves.clone(),
            pay_grade: job.pay_grade.clone(),
            sub_category: job.sub_category.clone(),
            category_raw: job.category_raw.clone(),
            employment_type: job.employment_type.clone(),
            work_hours: job.work_hours.clone(),
            worker_type: job.worker_type.clone(),
            job_profile: job.job_profile.clone(),
            supervisory_organization: job.supervisory_organization.clone(),
            target_hire_date: job.target_hire_date.clone(),
            no_of_available_openings: job.no_of_available_openings.clone(),
            grade_profile: job.grade_profile.clone(),
            recruiting_start_date: job.recruiting_start_date.clone(),
            job_level: job.job_level.clone(),
            job_family: job.job_family.clone(),
            job_type: job.job_type.clone(),
            is_evergreen: job.is_evergreen.clone(),
            standardised_country: job.standardised_country.clone(),
            run_date: job.run_date.clone(),
            run_id: job.run_id.clone(),
            address_locality: job.address_locality.clone(),
            address_region: job.address_region.clone(),
            address_country: job.address_country.clone(),
            postal_code: job.postal_code.clone(),
            job_summary: job.job_summary.clone(),
            created_at: chrono::Utc::now(), // Default current time since Job doesn't have created_at
        };
        job_map.insert(job.identifier.clone(), job_history);
    }

    // Create MatchWithJob objects
    let mut matches_with_jobs = Vec::new();
    for candidate_match in matches {
        let job = job_map.get(&candidate_match.job_identifier).cloned();
        let match_with_job = MatchWithJob {
            match_data: candidate_match.clone(),
            job,
            match_score_percent: (candidate_match.score * 100.0) as i32,
            match_date_formatted: candidate_match.created_at.format("%Y-%m-%d").to_string(),
        };
        matches_with_jobs.push(match_with_job);
    }

    // Sort by score descending (best matches first)
    matches_with_jobs.sort_by(|a, b| b.match_data.score.partial_cmp(&a.match_data.score).unwrap_or(std::cmp::Ordering::Equal));

    // Calculate statistics
    let matches_count = matches_with_jobs.len();
    let high_score_count = matches_with_jobs
        .iter()
        .filter(|m| m.match_data.score > 0.8)
        .count();
    let good_fit_count = matches_with_jobs
        .iter()
        .filter(|m| m.match_data.score > 0.6)
        .count();
    let has_matches = matches_count > 0;

    let template = DashboardTemplate {
        title: "Dashboard".to_string(),
        user_name: candidate.name,
        matches_count,
        high_score_count,
        good_fit_count,
        has_matches,
        matches_with_jobs,
    };

    Ok(Html(template.render()?))
}

pub async fn trigger_match(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, WebError> {
    let candidate = get_current_user(&session, &*state.db).await?;
    let candidate_id = candidate.id.unwrap_or(0);

    // Klone den State für den Hintergrund-Task
    let bg_state = state.clone();
    let profile_text = candidate.profile_text.clone();
    let oauth_sub = candidate.oauth_sub.clone();

    // Background Task (Fire and Forget)
    tokio::spawn(async move {
        tracing::info!("Starte asynchrone KI-Evaluierung für User: {}", oauth_sub);

        // 1. Hole aktuelle Jobs
        if let Ok(jobs) = bg_state.db.get_latest_jobs().await {
            // 2. Starte KI Matching
            if let Ok(matches) = bg_state.ai.match_candidate(&profile_text, jobs).await {
                // 3. Speichere Ergebnisse
                for match_data in matches {
                    let mut final_match = match_data.clone();
                    final_match.candidate_id = candidate_id;
                    let _ = bg_state.db.insert_candidate_match(&final_match).await;
                }
                tracing::info!("KI-Evaluierung für User {} abgeschlossen.", oauth_sub);
            } else {
                tracing::error!("KI-Matching fehlgeschlagen für User: {}", oauth_sub);
            }
        } else {
            tracing::error!("Konnte keine Jobs laden für User: {}", oauth_sub);
        }
    });

    // Setze eine Flash-Message für das UI (optional, falls implementiert)
    let _ = session.insert("success", true).await;

    // Sofortiger Redirect, während die KI im Hintergrund rechnet
    Ok(Redirect::to("/dashboard"))
}

pub async fn get_match_detail(
    Path(match_id): Path<i64>,
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    let candidate = get_current_user(&session, &*state.db).await?;

    // Rufe die Daten ab
    let (match_data, job) = state
        .db
        .get_match_detail(match_id)
        .await
        .map_err(WebError::Database)?
        .ok_or_else(|| WebError::Database(anyhow::anyhow!("Match not found")))?;

    // Sicherheitscheck: Gehört das Match dem User?
    if match_data.candidate_id != candidate.id.unwrap_or(0) {
        return Err(WebError::Auth("Unauthorized access to match".to_string()));
    }

    let template = MatchDetailTemplate {
        title: format!("Match: {}", job.title),
        user_name: candidate.name,
        match_data,
        job_title: job.title,
        job_location: job.location,
        job_description: job.description,
        job_organization: job.organization,
        job_employment_type: job.employment_type,
        job_level: job.job_level,
        job_family: job.job_family,
    };

    Ok(Html(template.render()?))
}

pub async fn get_jobs(
    Query(params): Query<JobQuery>,
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Try to get current user, but don't require authentication for public access
    let user_name = session.get::<String>("user_name").await.unwrap_or(None);

    // Pagination parameters
    let page = params.page.unwrap_or(1).max(1); // Default to page 1, minimum 1
    let limit = 20; // 20 items per page
    let offset = (page - 1) * limit;

    // Get jobs from database
    let (jobs, total_count) = state
        .db
        .get_jobs_paginated(limit, offset, params.q.clone())
        .await
        .map_err(WebError::Database)?;

    // Calculate pagination
    let total_pages = ((total_count as f64) / (limit as f64)).ceil() as i64;
    let total_pages = total_pages.max(1); // At least 1 page

    let search_query = params.q.clone().unwrap_or_default();
    let has_search = params.q.is_some();

    let template = JobsTemplate {
        title: "All Jobs".to_string(),
        user_name: user_name.unwrap_or_else(|| "Guest".to_string()),
        jobs,
        current_page: page,
        total_pages,
        search_query,
        total_count,
        has_search,
    };

    Ok(Html(template.render()?))
}

pub async fn get_job_detail(
    Path(identifier): Path<String>,
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Try to get current user, but don't require authentication for public access
    let user_name = session.get::<String>("user_name").await.unwrap_or(None);

    let job = state
        .db
        .get_job_by_identifier(&identifier)
        .await
        .map_err(WebError::Database)?
        .ok_or_else(|| WebError::Database(anyhow::anyhow!("Job not found")))?;

    let template = JobDetailTemplate {
        title: format!("Job: {}", job.title),
        user_name: user_name.unwrap_or_else(|| "Guest".to_string()),
        job,
    };

    Ok(Html(template.render()?))
}

// Router configuration
pub fn web_ui_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/profile", get(get_profile).post(post_profile))
        .route("/dashboard", get(get_dashboard))
        .route("/match/{id}", get(get_match_detail))
        .route("/job/{identifier}", get(get_job_detail))
        .route("/jobs", get(get_jobs))
        .route("/api/trigger-match", post(trigger_match))
}
