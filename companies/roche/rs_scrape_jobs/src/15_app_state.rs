use crate::ai_core::AiProvider;
use crate::db_traits::DatabaseProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<dyn DatabaseProvider>,
    pub ai: Arc<dyn AiProvider>,
}
