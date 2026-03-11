### Architektur-Vorgaben & Vorgehen
Damit der Code wartbar bleibt und die in der `Cargo.toml` definierten Features (`ai`, `scraper`, `db`, `web`) nicht unnötig hart gekoppelt werden, lagern wir den Ablauf für die AI-Annotation in ein neues Workflow-Modul aus. Der `pipeline_orchestrator` wird so angepasst, dass er als Abhängigkeit optional (per `Option<Arc<dyn AiProvider>>`) die AI entgegennimmt. So triggern sowohl die Admin-UI als auch das CLI-Scraping dieselbe Logik, ohne die Modulgrenzen zu verletzen.

---

### Implementierungsplan in detaillierten Schritten

#### Schritt 1: Baseline Verifikation
Führe vorab `cargo check --all-features` und `cargo test --all-features` aus, um zu verifizieren, dass der bisherige Code fehlerfrei ist.

#### Schritt 2: Erstellung des AI-Workflow Moduls
Erstelle eine neue Datei `src/08_ai_workflow.rs`. Diese Funktion holt Jobs ohne Summary, sendet sie an die Gemini-API und updatet die Einträge.

**Inhalt für `src/08_ai_workflow.rs`:**
```rust
#![cfg(feature = "ai")]

use std::sync::Arc;
use crate::db_traits::DatabaseProvider;
use crate::ai_core::AiProvider;

/// Holt unannotierte Jobs, generiert per AI Zusammenfassungen und speichert diese.
pub async fn annotate_unannotated_jobs(
    db: Arc<dyn DatabaseProvider>,
    ai: Arc<dyn AiProvider>,
    limit: usize,
) -> anyhow::Result<usize> {
    let unannotated = db.get_unannotated_jobs(limit).await?;
    if unannotated.is_empty() {
        return Ok(0);
    }
    
    let annotations = ai.annotate_jobs(unannotated.clone()).await?;
    let mut count = 0;
    
    for annotation in annotations {
        // Finde den passenden Job über den `idx` Index der AI Antwort
        if let Some(job) = unannotated.get(annotation.idx as usize) {
            // Konvertiere das Array aus Strings in eine formatierte Aufzählungsliste
            let formatted_summary = annotation.job_summary
                .iter()
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            
            db.update_job_ai(&job.identifier, &formatted_summary).await?;
            count += 1;
        }
    }
    
    Ok(count)
}
```

**In `src/lib.rs` (bzw. Haupt-Moduldatei):**
Füge die Referenz zum neuen Modul hinzu:
```rust
#[cfg(feature = "ai")]
#[path = "08_ai_workflow.rs"]
pub mod ai_workflow;
```

#### Schritt 3: Pipeline Orchestrator erweitern
Passe die `run_pipeline`-Funktion in `src/06_pipeline_orchestrator.rs` an, damit sie am Ende des Scrapings automatisch die AI-Annotation triggert.

**Änderungen in `src/06_pipeline_orchestrator.rs`:**
```rust
#[cfg(feature = "ai")]
use crate::ai_core::AiProvider;

// 1. Passe die Signatur an:
pub async fn run_pipeline(
    db: Arc<dyn crate::db_traits::DatabaseProvider>, 
    #[cfg(feature = "ai")] ai: Option<Arc<dyn AiProvider>>,
    #[cfg(not(feature = "ai"))] ai: Option<()>, // Fallback, wenn AI nicht kompiliert wird
    debug_mode: bool
) -> anyhow::Result<()> {
    
    // ...[Bestehender Code fürs Scraping bleibt unverändert] ...

    // 2. Füge ganz ans Ende, direkt VOR dem `Ok(())`, folgendes ein:
    #[cfg(feature = "ai")]
    if let Some(ai_provider) = ai {
        tracing::info!("Scraping abgeschlossen. Starte AI-Annotation für unannotierte Jobs...");
        match crate::ai_workflow::annotate_unannotated_jobs(db.clone(), ai_provider, 50).await {
            Ok(count) => tracing::info!("Pipeline AI-Schritt: Erfolgreich {} Jobs annotiert.", count),
            Err(e) => tracing::error!("Pipeline AI-Schritt fehlgeschlagen: {:?}", e),
        }
    }

    Ok(())
}
```

#### Schritt 4: CLI Entrypoint anpassen
Passe das CLI in `src/bin/main.rs` an, damit beim Kommandozeilen-Scraping die AI mit in die Pipeline gegeben wird.

**In `src/bin/main.rs` im Match-Arm `Commands::TriggerScrape`:**
```rust
        Commands::TriggerScrape { debug_dump } => {
            #[cfg(feature = "scraper")]
            {
                tracing::info!("Starting job scraping pipeline...");
                let db_provider = init_database(&config.db_path).await?;

                // NEU: AI Provider optional instanziieren
                #[cfg(feature = "ai")]
                let ai_provider: Option<Arc<dyn rs_scrape::ai_core::AiProvider>> = {
                    use rs_scrape::ai_gemini::GeminiProvider;
                    match GeminiProvider::new(&config.gemini_api_key) {
                        Ok(provider) => Some(Arc::new(provider)),
                        Err(e) => {
                            tracing::warn!("AI Provider Fehler. Überspringe AI-Annotation: {}", e);
                            None
                        }
                    }
                };
                
                #[cfg(not(feature = "ai"))]
                let ai_provider = None;

                // AUFRUF ANPASSEN:
                rs_scrape::pipeline_orchestrator::run_pipeline(
                    db_provider.clone(), 
                    ai_provider, 
                    debug_dump
                ).await?;

                tracing::info!("Scraping completed successfully!");
            }
        }
```

#### Schritt 5: Admin UI & Controller aktualisieren
Füge den Button in der Web-Oberfläche hinzu und verbinde ihn mit der Logik.

**1. In `templates/admin.html`:**
Füge unter der existierenden Scraping-Form folgendes an:
```html
    <form action="/admin/trigger-ai" method="POST" style="margin-top: 20px; padding-top: 20px; border-top: 1px solid #ddd;">
        <button type="submit" class="btn btn-secondary" {% if is_running %}disabled style="opacity: 0.5; cursor: not-allowed;"{% endif %}>
            🤖 Generate AI Summaries (Unannotated Jobs)
        </button>
    </form>
```

**2. In `src/17_admin.rs`:**
Passe zunächst den existierenden `run_pipeline`-Aufruf in `post_trigger_scrape` an:
```rust
// Signatur anpassen! Füge `Some(bg_state.ai.clone())` hinzu.
match pipeline_orchestrator::run_pipeline(bg_state.db.clone(), Some(bg_state.ai.clone()), debug_dump).await {
```

Füge dann den neuen POST-Handler für die manuelle AI-Annotation hinzu:
```rust
pub async fn post_trigger_ai(
    session: Session,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, WebError> {
    is_admin(&session, &state).await?;

    let bg_state = state.clone();
    tokio::spawn(async move {
        tracing::info!("Admin hat manuelle AI-Annotation getriggert.");
        
        #[cfg(feature = "ai")]
        match crate::ai_workflow::annotate_unannotated_jobs(bg_state.db.clone(), bg_state.ai.clone(), 50).await {
            Ok(count) => tracing::info!("Admin-Action: Erfolgreich {} Jobs annotiert.", count),
            Err(e) => tracing::error!("Admin-Action: AI Annotation fehlgeschlagen: {:?}", e),
        }
    });

    Ok(Redirect::to("/admin"))
}
```

Registriere den neuen Endpunkt in `admin_routes`:
```rust
pub fn admin_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", axum::routing::get(get_admin_dashboard))
        .route("/trigger", axum::routing::post(post_trigger_scrape))
        .route("/trigger-ai", axum::routing::post(post_trigger_ai)) // <--- HIER NEU
}
```

#### Schritt 6: Verifikation & Tests
Nachdem die KI alle Code-Schritte ausgeführt hat, sollen folgende Befehle zur Verifikation ausgeführt werden:
1. `cargo build --all-features` (Um sicherzustellen, dass Typen, Module und Traits korrekt matchen)
2. `cargo test --all-features` (Um zu validieren, dass z.B. der `web_integration_e2e`-Test nicht durch die API-Anpassungen von `run_pipeline` gebrochen wurde)
3. Starten des Servers lokal und Klicken des neuen Buttons in der Admin-Console, um im Terminal die Logs zu prüfen (`Admin hat manuelle AI-Annotation getriggert.`).