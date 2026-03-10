use crate::ai_core::AiProvider;
use crate::config::AppConfig;
use crate::db_traits::DatabaseProvider;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, PartialEq)]
pub enum ScrapeStatus {
    Idle,
    Running {
        start_time: DateTime<Utc>,
        debug_mode: bool,
    },
    Error(String),
    Success(DateTime<Utc>),
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<dyn DatabaseProvider>,
    pub ai: Arc<dyn AiProvider>,
    pub config: Arc<AppConfig>,
    pub scrape_status: Arc<RwLock<ScrapeStatus>>,
}
