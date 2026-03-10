use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_path: String,
    pub gemini_api_key: String,
    pub host: String,
    pub port: u16,
    pub is_debug: bool,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub oauth_redirect_url: String,
    pub session_secure: bool,
    pub session_max_age_days: u64,
    pub admin_username: String,
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
            github_client_id: env::var("GITHUB_CLIENT_ID")
                .expect("GITHUB_CLIENT_ID muss gesetzt sein"),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET")
                .expect("GITHUB_CLIENT_SECRET muss gesetzt sein"),
            oauth_redirect_url: env::var("OAUTH_REDIRECT_URL")
                .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
            session_secure: env::var("SESSION_SECURE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            session_max_age_days: env::var("SESSION_MAX_AGE_DAYS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            admin_username: env::var("ADMIN_USERNAME").unwrap_or_else(|_| "plops".to_string()),
        }
    }
}
