# Implementierungsplan: Rust-Port der Roche Job Scraper Pipeline

## 1. Spezifische Implementierungs-Vorgaben (Für das ausführende LLM)

### 1.1 Datenbank-Abstraktion (Vorbereitung für PostgreSQL)
Um von SQLite (aktuell `libsql`) später nahtlos auf PostgreSQL wechseln zu können, **muss** das *Repository Pattern* über Rust Traits (`async-trait`) implementiert werden. Die Geschäftslogik darf niemals direkte SQL-Queries ausführen.

**Vorgabe:** Erstelle eine Datei `src/01c_db_traits.rs`.
```rust
use async_trait::async_trait;
use crate::models::{Job, Candidate, CandidateMatch};
use anyhow::Result;

#[async_trait]
pub trait DatabaseProvider: Send + Sync {
    async fn insert_job_history(&self, job: &Job) -> Result<()>;
    async fn get_latest_jobs(&self) -> Result<Vec<Job>>;
    async fn upsert_candidate(&self, candidate: &Candidate) -> Result<i64>;
    async fn insert_candidate_match(&self, match_data: &CandidateMatch) -> Result<()>;
    async fn get_matches_for_candidate(&self, candidate_id: i64) -> Result<Vec<CandidateMatch>>;
}
```
*Anweisung:* Aktuell wird dieses Trait für SQLite implementiert (`01b_db_repo.rs`). Später muss nur eine neue Datei `01d_db_postgres.rs` geschrieben werden, die dasselbe Trait implementiert.

### 1.2 Scraper "Politeness" (Rate Limiting für HTTP)
Um Roche nicht zu überlasten, **muss** ein konfigurierbarer Delay eingebaut werden.
**Vorgabe:** In `04_downloader.rs` muss eine Pause zwischen den Requests mit `rand` (Jitter) und `tokio::time::sleep` implementiert werden.
```rust
use rand::Rng;
use tokio::time::{sleep, Duration};

async fn polite_delay(min_sec: u64, max_sec: u64) {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(min_sec..=max_sec);
    sleep(Duration::from_secs(delay)).await;
}
```

### 1.3 LLM Rate Limiting & Batching
Die KI-Anfragen müssen streng limitiert werden (Requests per Minute, Tokens per Minute, Requests per Day).
**Vorgabe:** Erstelle ein Config-Struct und einen Token-Tracker in `07d_ai_rate_limiter.rs`.

```rust
pub struct AiModelConfig {
    pub name: String,
    pub rpm_limit: u32,
    pub tpm_limit: u32,
    pub rpd_limit: u32,
    pub assumed_words_per_token: f32, // z.B. 0.75
}
// Für gemini-3.1-flash-lite-preview: rpm=15, tpm=250_000, rpd=500
```

---

## 2. Generelle Architektur & Programming Patterns für Rust

Für diesen Anwendungsfall eignen sich folgende Architektur-Muster am besten:

1.  **Ports and Adapters (Hexagonale Architektur):** 
    *   Wir trennen die reine Datenlogik (Domänenmodelle wie `Job`, `Candidate`) strikt von der Infrastruktur (Datenbank, Web-Scraper, AI-API).
    *   *Vorteil:* Du kannst z.B. die Datenbank testen, ohne das Web-Scraping-Modul überhaupt kompilieren zu müssen.
2.  **Trait-basierte Abstraktion (Strategy Pattern):**
    *   Anstatt harte Abhängigkeiten zu `rig` oder `genai` zu programmieren, definieren wir ein Trait `AiProvider`. So lässt sich das Agentic-Modell (Dateibasiert) und das API-Modell (Netzwerkbasiert) nahtlos austauschen.
3.  **Repository Pattern für die Datenbank:**
    *   Alle SQL-Statements werden gekapselt (siehe 1.1).
4.  **Producer-Consumer (Pipeline Pattern):**
    *   Für das Scraping und den Download: Ein Task sammelt URLs (Producer) und füttert sie über einen `tokio::sync::mpsc` Channel an mehrere asynchrone Worker (Consumer), die die HTML-Seiten parallel herunterladen.

---

## 3. Strategie für minimale Abhängigkeiten (Cargo Features)

Da Rust-Kompilierzeiten explodieren können, nutzen wir **Cargo Features**. Jedes Modul kann separat aktiviert und kompiliert werden.

**Beispiel `Cargo.toml`:**
```toml
[package]
name = "rs-scrape"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
log = "0.4"

# Optionale Abhängigkeiten (nur kompiliert, wenn Feature aktiv)
libsql = { version = "0.3", optional = true }
chromiumoxide = { version = "0.5", optional = true }
reqwest = { version = "0.11", optional = true }
rig-core = { version = "0.1", optional = true }
axum = { version = "0.7", optional = true }
askama = { version = "0.12", optional = true }

[features]
default = ["db", "scraper", "ai", "web"] # Für den finalen Build
db = ["dep:libsql"]
scraper = ["dep:chromiumoxide", "dep:reqwest"]
ai = ["dep:rig-core"]
web = ["dep:axum", "dep:askama"]
```

---

## 4. Dateistruktur (Nummeriertes System)

Die `src/`-Hierarchie wird so aufgebaut, dass man die Ausführungsreihenfolge der Pipeline sofort erkennt. Zu Testzwecken legen wir für jede Stufe eine Datei im Verzeichnis `src/bin/` an.

```text
rs_scrape_jobs/
├── Cargo.toml
└── src/
    ├── lib.rs                   # Exportiert alle Module
    ├── 00_models.rs             # Reine Datenstrukturen (Job, Candidate, Match)
    ├── 01_db_setup.rs           # Tabellen Initialisierung
    ├── 01b_db_repo.rs           # SQLite CRUD Operationen
    ├── 01c_db_traits.rs         # DatabaseProvider Trait
    ├── 02_web_core.rs           # Basis-Browser-Steuerung (Chromiumoxide)
    ├── 03_scraper_roche.rs      # Spezifische Roche-Navigation
    ├── 04_downloader.rs         # HTML-Download (mit Politeness)
    ├── 05_json_extractor.rs     # Regex/String-Suche nach phApp.ddo
    ├── 06_pipeline_orchestrator.rs # In-Memory Koordination (ohne Datei-IO)
    ├── 07_ai_core.rs            # AiProvider Trait
    ├── 07b_ai_gemini.rs         # Rig / GenAI Implementierung
    ├── 07d_ai_rate_limiter.rs   # RPM/TPM Limitierung
    ├── 07e_ai_batch_builder.rs  # Wortzähler und Batch-Generierung
    ├── 11_web_server.rs         # Axum Webserver & Session Handling
    ├── 12_auth.rs               # OAuth Logik
    ├── 13_web_ui.rs             # Askama Templates & Routen
    ├── 14_scheduler.rs          # Tokio Nightly Cron-Job
    └── bin/                     # ISOLIERTE TEST-BINARIES
        ├── stage1_db.rs
        ├── stage4_download.rs
        ├── stage6_ai_test.rs
        ├── stage10_web.rs
        └── main.rs              # Das finale CLI (Clap)
```

---

## 5. Der Stufenweise Implementierungs- & Testplan

#### Stufe 1: Datenmodelle & Datenbank-Infrastruktur
*   **Aktion:** Erstellen von `00_models.rs`, `01_db_setup.rs` und `01b_db_repo.rs`.
*   **Fokus:** SQLite-Datei lokal anlegen, Tabellen erstellen, Dummy-Daten schreiben und lesen.
*   **Kompilieren mit:** `cargo run --bin stage1_db --features "db"`
*   **Ziel:** Du siehst eine `jobs_minutils.db` in deinem Ordner, die du mit einem DB-Browser öffnen kannst. Alles läuft pfeilschnell via `libsql`.

#### Stufe 2: Web-Automatisierung Basis ("Proof of Life")
*   **Aktion:** Erstellen von `02_web_core.rs`.
*   **Fokus:** `chromiumoxide` starten, Google aufrufen, Screenshot machen oder Seitentitel in der Konsole ausgeben.
*   **Kompilieren mit:** `cargo run --bin stage2_browser_test --features "scraper"`
*   **Ziel:** Sicherstellen, dass Headless-Chrome auf dem System sauber angesteuert werden kann.

#### Stufe 3: Navigation & Suche auf Roche Careers
*   **Aktion:** Erstellen von `03_scraper_roche.rs`.
*   **Fokus:** Navigation zu Roche. Klick auf Cookie-Banner. Klick auf "Schweiz". Iteration durch die Paginierung, um rohe href-Links zu extrahieren. Noch **kein** Download der Job-Detailseiten!
*   **Kompilieren mit:** `cargo run --bin stage3_search --features "scraper"`
*   **Ziel:** Liste aller Schweiz-Job-URLs im Terminal.

#### Stufe 4: Async Download & JSON Extraktion (Trennung von DB)
*   **Aktion:** Erstellen von `04_downloader.rs` und `05_json_extractor.rs`.
*   **Fokus:** URLs asynchron per HTTP GET laden. Das JSON (`phApp.ddo`) extrahieren.
*   **Kompilieren mit:** `cargo run --bin stage4_download --features "scraper"`
*   **Ziel:** Speichern der extrahierten JSON-Strings als einfache `.json` Dateien im Ordner `jobs_html/`. **Hier noch kein DB-Schreiben!**

#### Stufe 5: Parsing & Datenbank-Ingestion
*   **Aktion:** Erstellen von `06_data_ingestion.rs`.
*   **Fokus:** Wir lesen die JSON-Dateien von der Festplatte. Wir nutzen `serde_json`, parsen sie und schreiben sie in die SQLite.
*   **Test:** Wir lesen JSON -> Wir prüfen Logausgaben -> Wir schreiben in die DB.
*   **Ziel:** Eine gefüllte, relationale SQLite-Datenbank.

#### Stufe 6: KI-Infrastruktur & Job-Annotation
*   **Aktion:** Erstellen von `07_ai_core.rs` (Traits) und `07b_ai_gemini.rs`.
*   **Fokus:** Implementierung der `rig` Library. Ungelesene Jobs (z.B. 10 Stück) werden an Gemini gesendet, um generelle Relevanz zu scoren.
*   **Kompilieren mit:** `cargo run --bin stage6_ai_test --features "db ai"`
*   **Ziel:** Update der SQLite mit den AI-Annotationen. Strikte Typsicherheit (JSON-Array wird erzwungen).

#### Stufe 7: Historisiertes DB-Schema & Traits
*   **Aktion:** Erstelle `src/01c_db_traits.rs`, erweitere `src/00_models.rs` und `src/01_db_setup.rs`.
*   **Fokus:** 
    1. Definiere das `DatabaseProvider` Trait.
    2. Erweitere das SQLite Setup: 
       - Tabelle `candidates` (id, oauth_sub UNIQUE, name, profile_text).
       - Tabelle `candidate_matches` (id, candidate_id, job_identifier, model_used, score, explanation, created_at).
       - Ändere die Job-Speicherung so ab, dass bei einem Update (z.B. geänderte Beschreibung) ein neuer Datensatz in einer `job_history` Tabelle angelegt wird.
*   **Test:** Schreibe `bin/stage7_db_v2.rs`, um Dummy-Kandidaten und mehrere Matches zu speichern und mit `ORDER BY created_at DESC LIMIT 1` abzufragen.

#### Stufe 8: Scraper Politeness & In-Memory Pipeline
*   **Aktion:** Erweitere `04_downloader.rs` und erstelle `06_pipeline_orchestrator.rs`.
*   **Fokus:**
    1. Baue `polite_delay` (z.B. 20-60 Sekunden Pause) in den Downloader ein.
    2. Verbinde den Downloader direkt mit dem JSON-Extractor, ohne Dateien auf die Festplatte zu schreiben (In-Memory).
    3. **Debug-Logik:** Wenn die Pipeline mit `--debug-dump` gestartet wird, speichere rohe HTML/JSON-Dateien unter `debug_dumps/YYYY-MM-DD/`.
*   **Test:** `bin/stage8_polite_scrape.rs`.

#### Stufe 9: Smart Batching & Token Limiting für Gemini
*   **Aktion:** Erstelle `07d_ai_rate_limiter.rs` und `07e_ai_batch_builder.rs`.
*   **Fokus:** 
    1. Implementiere Logik für die Gemini-Modelle aus der Konfigurationstabelle.
    2. Iteriere über Jobs, zähle die Wörter (`text.split_whitespace().count()`). Ein Wort = ~1.33 Token.
    3. Wenn das Limit von `tpm_limit * 0.8` (Sicherheitspuffer) erreicht ist, wird der Request gesendet.
    4. Nutze `tokio::time::sleep`, wenn das `rpm` (Requests per Minute) Limit erreicht ist.
*   **Test:** `bin/stage9_rate_limit_test.rs`.

#### Stufe 10: Axum Web-Server & Authentifizierung
*   **Aktion:** Erstelle `11_web_server.rs` und `12_auth.rs`.
*   **Fokus:**
    1. Nutze `axum` und `tower-sessions`.
    2. Richte OAuth2 (GitHub oder Google) ein.
    3. Erstelle minimale Routen: `/login`, `/auth/callback`, `/logout`.
    4. Speichere den Nutzer über das `DatabaseProvider` Trait in der `candidates` Tabelle.
*   **Test:** `bin/stage10_web.rs` (Sollte lokal auf Port 3000 lauschen).

#### Stufe 11: Web-UI (Askama Templates)
*   **Aktion:** Erstelle Ordner `templates/`, erstelle `13_web_ui.rs`.
*   **Fokus:**
    1. Nutze Askama für HTML-Rendering.
    2. **Route `/profile`:** Ein Formular für den Lebenslauf (Text) des Users.
    3. **Route `/dashboard`:** Zeigt eine formatierte Liste der neuesten Auswertungen (`candidate_matches`), gefiltert für den aktuellen User.
    4. Button "Neu bewerten", der einen asynchronen AI-Task anwirft.

#### Stufe 12: Nightly Cron-Job
*   **Aktion:** Erstelle `14_scheduler.rs`.
*   **Fokus:**
    1. Nutze `tokio-cron-scheduler`.
    2. Richte einen Job ein, der jede Nacht triggert.
    3. Ablauf: Scrape Jobs -> Finde Deltas -> Speichere in DB -> Hole alle Kandidaten -> Generiere AI Matches (mit Politeness & Rate Limiting) -> Speichere in DB.
*   **Test:** `bin/stage12_cron_test.rs` (Mit einem minütlichen Trigger zum Testen).

#### Stufe 13: Main CLI Zusammenführung
*   **Aktion:** Überarbeite `bin/main.rs`.
*   **Fokus:** Nutze `clap` für das finale CLI:
    *   `rs-scrape serve`: Startet Webserver UND den Background-Cron-Job.
    *   `rs-scrape trigger-scrape --debug-dump`: Startet den Scraper sofort im Debug-Modus.
    *   `rs-scrape force-match --candidate-id <ID>`: Zwingt die KI zur sofortigen Neubewertung.