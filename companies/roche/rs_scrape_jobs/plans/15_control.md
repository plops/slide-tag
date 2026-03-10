**PROMPT FÜR DIE IMPLEMENTIERUNGS-KI:**

Du agierst als Senior Rust Developer. Dein Tech-Lead (Architekt) hat den folgenden Implementierungsplan entworfen, um ein "Admin-Dashboard" in unsere Axum-Anwendung einzubauen. Dieses Dashboard soll es einem Admin (GitHub User: `plops`) erlauben, den Scraper im laufenden Betrieb über die Web-Oberfläche zu triggern, ohne den Server stoppen zu müssen.

Lies dir die Architektur-Vorgaben genau durch und implementiere sie Schritt für Schritt.

#### Schritt 1: Konfiguration & AppState anpassen
Wir müssen den Status des Scrapers im RAM halten, damit das Dashboard anzeigen kann, ob gerade ein Scrape läuft. Zudem muss der `AppConfig` in den `AppState` aufgenommen werden, um Berechtigungen prüfen zu können.

**1.1 In `src/08_config.rs`:**
Füge ein Feld für den Admin-User hinzu:
```rust
// In AppConfig struct hinzufügen:
pub admin_username: String,

// In AppConfig::from_env() hinzufügen:
admin_username: env::var("ADMIN_USERNAME").unwrap_or_else(|_| "plops".to_string()),
```

**1.2 In `src/15_app_state.rs`:**
Erweitere den `AppState`, damit wir den Scraper-Status überwachen können.
```rust
use crate::ai_core::AiProvider;
use crate::db_traits::DatabaseProvider;
use crate::config::AppConfig;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub enum ScrapeStatus {
    Idle,
    Running { start_time: DateTime<Utc>, debug_mode: bool },
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
```

**1.3 In `src/bin/main.rs` und `src/bin/stage11_web_ui.rs` etc.:**
Korrigiere die Initialisierung von `AppState`, indem du `config: Arc::new(config.clone())` und `scrape_status: Arc::new(RwLock::new(ScrapeStatus::Idle))` hinzufügst.

#### Schritt 2: Refactoring des Pipeline Orchestrators
Aktuell ist die Methode in `06b_pipeline_orchestrator.rs` fest an den `JobRepository` Typ gebunden. Da der Webserver aber mit dem Trait `Arc<dyn DatabaseProvider>` arbeitet, müssen wir das entkoppeln.

**In `src/06b_pipeline_orchestrator.rs`:**
Ändere die Signatur von:
`pub async fn run_pipeline(repo: &JobRepository, debug_dump: bool) -> Result<()>`
zu:
```rust
// WICHTIG: Importiere das Trait!
use crate::db_traits::DatabaseProvider;
use std::sync::Arc;

pub async fn run_pipeline(repo: Arc<dyn DatabaseProvider>, debug_dump: bool) -> Result<()> {
    // ... restlicher Code bleibt gleich, nur Methodenaufrufe auf repo 
    // ändern sich von repo.insert_job_history(...) zu repo.insert_job_history(...)
    // da Arc<dyn Trait> Deref implementiert.
```
*Achtung:* Du musst ggf. auch in `src/bin/stage8_polite_scrape.rs` und `src/bin/main.rs` (beim TriggerScrape command) den Aufruf anpassen, sodass ein `Arc` übergeben wird.

#### Schritt 3: Den Admin-Controller erstellen
Erstelle eine neue Datei `src/17_admin.rs`. Hier bauen wir die Logik für das Dashboard und den Trigger.

```rust
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use std::sync::Arc;
use tower_sessions::Session;
use askama::Template;
use chrono::Utc;
use serde::Deserialize;

use crate::{app_state::{AppState, ScrapeStatus}, pipeline_orchestrator, web_ui::WebError};

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminTemplate {
    pub title: String,
    pub user_name: String,
    pub status: String,
    pub is_running: bool,
    pub app_version: &'static str,
}

#[derive(Deserialize)]
pub struct TriggerScrapeForm {
    #[serde(default)]
    pub debug_dump: String, // Checkbox sendet "on" oder existiert nicht
}

// Helfer zur Admin-Verifizierung
async fn is_admin(session: &Session, state: &AppState) -> Result<String, WebError> {
    let user_name = session.get::<String>("user_name").await
        .map_err(|_| WebError::Auth("Session error".to_string()))?
        .unwrap_or_default();
    
    if user_name.to_lowercase() != state.config.admin_username.to_lowercase() {
        return Err(WebError::Auth("Access Denied: Admin only".to_string()));
    }
    Ok(user_name)
}

pub async fn get_admin_dashboard(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    let user_name = is_admin(&session, &state).await?;
    
    let status_lock = state.scrape_status.read().await;
    let (status_text, is_running) = match &*status_lock {
        ScrapeStatus::Idle => ("Idle (Ready to scrape)".to_string(), false),
        ScrapeStatus::Running { start_time, debug_mode } => (
            format!("Running since {} (Debug: {})", start_time.format("%H:%M:%S"), debug_mode),
            true
        ),
        ScrapeStatus::Success(time) => (format!("Last success at {}", time.format("%H:%M:%S")), false),
        ScrapeStatus::Error(msg) => (format!("Error: {}", msg), false),
    };

    let template = AdminTemplate {
        title: "Admin Dashboard".to_string(),
        user_name,
        status: status_text,
        is_running,
        app_version: env!("CARGO_PKG_VERSION"),
    };

    Ok(Html(template.render()?))
}

pub async fn post_trigger_scrape(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<TriggerScrapeForm>,
) -> Result<Redirect, WebError> {
    is_admin(&session, &state).await?;

    let debug_dump = form.debug_dump == "on";

    // Status prüfen und setzen
    {
        let mut status_lock = state.scrape_status.write().await;
        if let ScrapeStatus::Running { .. } = *status_lock {
            return Ok(Redirect::to("/admin?error=already_running"));
        }
        *status_lock = ScrapeStatus::Running { 
            start_time: Utc::now(), 
            debug_mode: debug_dump 
        };
    }

    // Task im Hintergrund starten! Fire and forget.
    let bg_state = state.clone();
    tokio::spawn(async move {
        tracing::info!("Admin triggered manual scrape. Debug: {}", debug_dump);
        
        match pipeline_orchestrator::run_pipeline(bg_state.db.clone(), debug_dump).await {
            Ok(_) => {
                tracing::info!("Manual scrape completed successfully.");
                let mut status_lock = bg_state.scrape_status.write().await;
                *status_lock = ScrapeStatus::Success(Utc::now());
            }
            Err(e) => {
                tracing::error!("Manual scrape failed: {:?}", e);
                let mut status_lock = bg_state.scrape_status.write().await;
                *status_lock = ScrapeStatus::Error(e.to_string());
            }
        }
    });

    Ok(Redirect::to("/admin"))
}

pub fn admin_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", axum::routing::get(get_admin_dashboard))
        .route("/trigger", axum::routing::post(post_trigger_scrape))
}
```

#### Schritt 4: Template für das Admin-Dashboard
Erstelle die Datei `templates/admin.html`.

```html
{% extends "base.html" %}

{% block title %}Admin Dashboard{% endblock %}

{% block content %}
<div class="card">
    <h1>🛠️ Admin Dashboard</h1>
    <p>Welcome, {{ user_name }}. Here you can manage the application state and trigger jobs.</p>
</div>

<div class="card">
    <h2>Roche Job Scraper Control</h2>
    
    <div style="margin: 20px 0; padding: 15px; background: #f8f9fa; border-left: 4px solid #667eea; border-radius: 4px;">
        <strong>Current Status:</strong> 
        <span style="color: {% if is_running %}#ff9800{% else %}#28a745{% endif %}; font-weight: bold;">
            {{ status }}
        </span>
    </div>

    <form action="/admin/trigger" method="POST" style="margin-top: 20px;">
        <div class="form-group" style="display: flex; align-items: center; gap: 10px; margin-bottom: 20px;">
            <input type="checkbox" id="debug_dump" name="debug_dump" style="width: 20px; height: 20px;" {% if is_running %}disabled{% endif %}>
            <label for="debug_dump" style="margin: 0; cursor: pointer;">
                <strong>Enable Debug Dump</strong> (Saves raw HTML/JSON to disk. Uses lots of space!)
            </label>
        </div>

        <button type="submit" class="btn btn-primary" {% if is_running %}disabled style="opacity: 0.5; cursor: not-allowed;"{% endif %}>
            {% if is_running %}
                Scrape in Progress...
            {% else %}
                🚀 Trigger Manual Scrape Now
            {% endif %}
        </button>
    </form>
</div>
{% endblock %}
```

#### Schritt 5: Routing und Lib.rs aktualisieren

**5.1 In `src/lib.rs`:**
```rust
#[path = "17_admin.rs"]
pub mod admin;
```

**5.2 In `src/11_web_server.rs`:**
Importiere das Admin-Modul:
```rust
use crate::admin;
```

Füge die Admin-Routen in den Router ein, der den State benötigt (direkt neben `db_routes`):
```rust
    // Router for routes that need database state
    let db_routes = Router::new()
        .route("/profile", get(web_ui::get_profile).post(web_ui::post_profile))
        .route("/dashboard", get(web_ui::get_dashboard))
        .route("/match/{id}", get(web_ui::get_match_detail))
        .route("/job/{identifier}", get(web_ui::get_job_detail))
        .route("/jobs", get(web_ui::get_jobs))
        .route("/api/trigger-match", post(web_ui::trigger_match))
        .nest("/admin", admin::admin_routes()) // HIER HINZUFÜGEN
        .with_state(app_state.clone());
```

#### Schritt 6: Link im Navbar Template (Optional)
In `templates/base.html` kannst du im `nav_links` Block (sofern du willst, dass der Admin es leicht findet) einen Link einbauen:
```html
{% if user_name == "plops" %}
    <li><a href="/admin">⚙️ Admin</a></li>
{% endif %}
```

---
**Abschließender Hinweis für die KI:**
Führe nach der Implementierung unbedingt `./build.sh` aus, um sicherzustellen, dass keine Borrow-Checker Fehler durch die `Arc<dyn DatabaseProvider>` Umstellung im `run_pipeline` entstanden sind. Achte darauf, dass im `run_pipeline` keine Methoden mehr als immutable references (`&repo`) erwartet werden, da wir jetzt Klone von `Arc` übergeben!