use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_path: String,
    pub gemini_api_key: String,
    pub host: String,
    pub port: u16,
    pub is_debug: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); // Lade .env Datei falls vorhanden
        Self {
            db_path: env::var("DB_PATH").unwrap_or_else(|_| "jobs_minutils.db".to_string()),
            gemini_api_key: env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY muss gesetzt sein"),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            is_debug: env::var("DEBUG")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}
