# Master-Plan: Finale Integration der Roche Job Scraper Pipeline

**Kontext für das ausführende LLM:**
Du bist ein Rust-Experte. Deine Aufgabe ist es, die isolierten Module dieses Projekts zu einem lauffähigen Gesamtsystem zu integrieren. Nutze das `deepwiki MCP`, falls du die Dokumentation für `axum` (State Management), `tokio` (Spawning) oder `tracing` benötigst. Halte dich strikt an die vorgegebenen Datenstrukturen und Programmiermuster.

## 1. Architektur-Vorgaben & Programmiermuster

Wir nutzen **Dependency Injection via Global AppState**. Da wir asynchron mit `tokio` und `axum` arbeiten, müssen alle geteilten Ressourcen (Datenbank und KI-Provider) threadsicher in einem `Arc` (Atomic Reference Counted) verpackt werden.

Zudem nutzen wir das **Fire-and-Forget Pattern (Background Tasks)** für langlaufende KI-Aufgaben im Web-Server, damit HTTP-Requests nicht blockieren.

---

## 2. Implementierungs-Schritte (Exakt in dieser Reihenfolge abarbeiten)

### Schritt 1: Zentrales Logging & Konfiguration einrichten

Aktuell fehlen ein sauberes Logging und eine zentrale Konfiguration.

**Aufgabe:**

1. Füge der `Cargo.toml` die Dependencies `tracing = "0.1"`, `tracing-subscriber = "0.3"` und `dotenvy = "0.15"` hinzu.
2. Erstelle eine neue Datei `src/08_config.rs`.

**Code-Vorgabe für `src/08_config.rs`:**

```rust
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
            port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap_or(3000),
            is_debug: env::var("DEBUG").unwrap_or_else(|_| "false".to_string()).parse().unwrap_or(false),
        }
    }
}

```

### Schritt 2: Den AppState definieren (Dependency Injection)

Der Webserver (`11_web_server.rs`) und das UI (`13_web_ui.rs`) benötigen Zugriff auf die DB *und* die KI.

**Aufgabe:**
Erstelle eine neue Datei `src/15_app_state.rs` (und exportiere sie in `lib.rs`).

**Code-Vorgabe für `src/15_app_state.rs`:**

```rust
use std::sync::Arc;
use crate::db_traits::DatabaseProvider;
use crate::ai_core::AiProvider;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<dyn DatabaseProvider>,
    pub ai: Arc<dyn AiProvider>,
}

```

### Schritt 3: Axum Web-Server & UI anpassen

Der Webserver muss den neuen `AppState` nutzen anstelle des direkten `Arc<dyn DatabaseProvider>`.

**Aufgaben:**

1. In `src/11_web_server.rs`: Ändere die Signatur von `create_app` und `run_server`, sodass sie `app_state: Arc<AppState>` entgegennehmen. Ändere `.with_state(db_provider)` zu `.with_state(app_state.clone())`.
2. In `src/13_web_ui.rs`: Ändere alle `State(db_provider): State<Arc<dyn DatabaseProvider>>` zu `State(state): State<Arc<AppState>>`. Du greifst dann via `state.db` auf die Datenbank zu.
3. In `src/13_web_ui.rs` bei `trigger_match`: Implementiere das **Fire-and-Forget Pattern**.

**Code-Vorgabe für `trigger_match` in `13_web_ui.rs`:**

```rust
pub async fn trigger_match(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, WebError> {
    let candidate = get_current_user(&session, &state.db).await?;
    let candidate_id = candidate.id.unwrap_or(0);
    
    // Klone den State für den Hintergrund-Task
    let bg_state = state.clone();
    let profile_text = candidate.profile_text.clone();
    
    // Background Task (Fire and Forget)
    tokio::spawn(async move {
        tracing::info!("Starte asynchrone KI-Evaluierung für User: {}", candidate.oauth_sub);
        
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
                tracing::info!("KI-Evaluierung für User {} abgeschlossen.", candidate.oauth_sub);
            }
        }
    });

    // Setze eine Flash-Message für das UI (optional, falls implementiert)
    let _ = session.insert("success", true).await;
    
    // Sofortiger Redirect, während die KI im Hintergrund rechnet
    Ok(Redirect::to("/dashboard"))
}

```

### Schritt 4: Den Scheduler vernetzen (Mocks entfernen)

In `14_scheduler.rs` laufen aktuell Dummy-Funktionen. Diese müssen durch die echte Geschäftslogik ersetzt werden.

**Aufgabe:**

1. Ändere die `NightlyScheduler::new` Signatur, sodass sie `state: Arc<AppState>` entgegennimmt und speichert.
2. Lösche die Mock-Funktionen (`scrape_jobs_and_store`, `get_all_candidates`, etc.).
3. Passe `execute_nightly_pipeline` an.

**Code-Vorgabe für die Pipeline im Scheduler:**

```rust
async fn execute_nightly_pipeline(config: SchedulerConfig, state: Arc<AppState>) -> Result<()> {
    tracing::info!("Starte Nightly Pipeline...");

    // 1. Scrape Jobs (Echter Aufruf)
    // Hinweis: Repo Pattern muss hier beachtet werden.
    // Falls pipeline_orchestrator den konkreten JobRepository Typ braucht,
    // muss das in 06b_pipeline_orchestrator.rs auf das Trait DatabaseProvider umgebaut werden!
    // Für jetzt nehmen wir an, der Orchestrator nimmt `&*state.db`.
    // crate::pipeline_orchestrator::run_pipeline(&*state.db, config.debug).await?;

    // 2. Hole Kandidaten und Jobs
    // (Anmerkung für das LLM: get_all_candidates existiert noch nicht im Trait, 
    // bitte ergänze es in 01c_db_traits.rs und 01b_db_repo.rs: 
    // `async fn get_all_candidates(&self) -> Result<Vec<Candidate>>;`)
    
    let candidates = state.db.get_all_candidates().await?;
    let jobs = state.db.get_latest_jobs().await?;

    if candidates.is_empty() || jobs.is_empty() {
        tracing::info!("Keine Kandidaten oder Jobs vorhanden. Beende Pipeline.");
        return Ok(());
    }

    // 3. KI Matching durchführen (mit Limitierung)
    for candidate in candidates {
        tracing::info!("Berechne Matches für {}", candidate.name);
        if let Ok(matches) = state.ai.match_candidate(&candidate.profile_text, jobs.clone()).await {
             for match_data in matches {
                  let mut final_match = match_data.clone();
                  final_match.candidate_id = candidate.id.unwrap();
                  let _ = state.db.insert_candidate_match(&final_match).await;
             }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(config.batch_delay_seconds)).await;
    }

    tracing::info!("Nightly Pipeline erfolgreich beendet.");
    Ok(())
}

```

### Schritt 5: Die `main.rs` zusammenbauen

Jetzt wird alles in der CLI-Entrypoint Datei verdrahtet.

**Aufgabe:**
Passe `main.rs` an, um die Config zu laden, den Logger zu starten, den `AppState` zu bauen und Server sowie Scheduler zu starten.

**Beispiel-Ablauf für `main.rs` (Serve Command):**

1. Initialisiere `tracing_subscriber::fmt::init()`.
2. Lade `AppConfig::from_env()`.
3. Erstelle `db_provider = init_database(&config.db_path).await?`.
4. Erstelle `ai_provider = Arc::new(GeminiProvider::new(&config.gemini_api_key)?)`.
5. Erstelle `app_state = Arc::new(AppState { db: db_provider, ai: ai_provider })`.
6. Übergebe `app_state.clone()` an den Webserver und an den `NightlyScheduler`.

---

### Anweisung an dich als ausführendes LLM:

Verwende diesen Plan als Checkliste. Beginne mit **Schritt 1 und 2** (Config, Tracing und AppState).
Wenn du bei der Implementierung auf Typ-Fehler stößt (insbesondere bei `axum::extract::State` oder den `async_trait` Bounds wie `Send + Sync`), nutze das `deepwiki MCP`, um die aktuelle Dokumentation zu konsultieren.

