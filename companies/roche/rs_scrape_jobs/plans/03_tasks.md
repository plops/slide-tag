# Architektur- & Implementierungs-Vorgaben (Für das ausführende LLM)

## 1. Datenbank-Abstraktion (Vorbereitung für PostgreSQL)
Um von SQLite (aktuell `libsql`) später nahtlos auf PostgreSQL wechseln zu können, **muss** das *Repository Pattern* über Rust Traits (`async-trait`) implementiert werden. Die Geschäftslogik darf niemals direkte SQL-Queries ausführen.

**Vorgabe an das LLM:** Erstelle eine Datei `src/01c_db_traits.rs`.
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
*Anweisung:* Aktuell wird dieses Trait für SQLite implementiert. Später muss nur eine neue Datei `01d_db_postgres.rs` geschrieben werden, die dasselbe Trait implementiert.

## 2. Scraper "Politeness" (Rate Limiting für HTTP)
Um Roche nicht zu überlasten, **muss** ein konfigurierbarer Delay eingebaut werden.
**Vorgabe an das LLM:** In `04_downloader.rs` muss eine Pause zwischen den Requests mit `rand` (Jitter) und `tokio::time::sleep` implementiert werden.
```rust
// Beispiel-Snippet für das LLM
use rand::Rng;
use tokio::time::{sleep, Duration};

async fn polite_delay(min_sec: u64, max_sec: u64) {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(min_sec..=max_sec);
    sleep(Duration::from_secs(delay)).await;
}
```

## 3. LLM Rate Limiting & Batching
Die KI-Anfragen müssen streng limitiert werden (RPM, TPM, RPD). 
**Vorgabe an das LLM:** Erstelle ein Config-Struct und einen Token-Tracker in `07d_ai_rate_limiter.rs`.

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


### 1. Architektur & Programming Patterns für Rust

Für diesen Anwendungsfall eignen sich folgende Architektur-Muster am besten:

1.  **Ports and Adapters (Hexagonale Architektur):** 
    *   Wir trennen die reine Datenlogik (Domänenmodelle wie `Job`, `Candidate`) strikt von der Infrastruktur (Datenbank, Web-Scraper, AI-API).
    *   *Vorteil:* Du kannst z.B. die Datenbank testen, ohne das Web-Scraping-Modul überhaupt kompilieren zu müssen.
2.  **Trait-basierte Abstraktion (Strategy Pattern):**
    *   Anstatt harte Abhängigkeiten zu `rig` oder `genai` zu programmieren, definieren wir ein Trait `AiProvider`. So lässt sich das Agentic-Modell (Dateibasiert) und das API-Modell (Netzwerkbasiert) nahtlos austauschen.
3.  **Repository Pattern für die Datenbank:**
    *   Alle SQL-Statements werden in einem Struktur-Typ `JobRepository` gekapselt.
4.  **Producer-Consumer (Pipeline Pattern):**
    *   Für das Scraping und den Download: Ein Task sammelt URLs (Producer) und füttert sie über einen `tokio::sync::mpsc` Channel an mehrere asynchrone Worker (Consumer), die die HTML-Seiten parallel herunterladen.

---

### 2. Strategie für minimale Abhängigkeiten (Cargo Features)

Da Rust-Kompilierzeiten bei vielen Abhängigkeiten (`chromiumoxide`, `typst`, `reqwest`) explodieren können, nutzen wir **Cargo Features**. Jedes Modul kann separat aktiviert und kompiliert werden.

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
askama = { version = "0.12", optional = true }
typst = { version = "0.11", optional = true }

[features]
default = ["db", "scraper", "ai", "pdf"] # Für den finalen Build
db = ["dep:libsql"]
scraper = ["dep:chromiumoxide", "dep:reqwest"]
ai = ["dep:rig-core"]
pdf = ["dep:askama", "dep:typst"]
```

---

### 3. Dateistruktur (Nummeriertes System)

Die `src/`-Hierarchie wird so aufgebaut, dass man die Ausführungsreihenfolge der Pipeline sofort erkennt. Zu Testzwecken legen wir für jede Stufe eine Datei im Verzeichnis `src/bin/` an. So kann jede Stufe als **eigenständiges Programm** isoliert ausgeführt und getestet werden.

```text
rs_scrape_jobs/
├── Cargo.toml
└── src/
    ├── lib.rs                   # Exportiert alle Module
    ├── 00_models.rs             # Reine Datenstrukturen (Job, Skill, Location)
    ├── 01_db_setup.rs           # LibSQL Initialisierung & Tabellen
    ├── 01b_db_repo.rs           # CRUD Operationen (Insert, Select)
    ├── 02_web_core.rs           # Basis-Browser-Steuerung (Chromiumoxide)
    ├── 03_scraper_roche.rs      # Spezifische Roche-Navigation & Pagination
    ├── 04_downloader.rs         # Paralleler HTML-Download via Reqwest
    ├── 05_json_extractor.rs     # Regex/String-Suche nach phApp.ddo
    ├── 06_data_ingestion.rs     # Mapping von JSON zu 00_models + DB Insert
    ├── 07_ai_core.rs            # AiProvider Trait Definition
    ├── 07b_ai_gemini.rs         # Rig / GenAI Implementierung
    ├── 07c_ai_agentic.rs        # Dateibasierte IN/OUT Implementierung
    ├── 08_matcher.rs            # Pipeline-Logik für Kandidaten-Matching
    ├── 09_templating.rs         # Askama Typst-Templates
    ├── 10_typst_compiler.rs     # Native PDF Generierung
    └── bin/                     # ISOLIERTE TEST-BINARIES
        ├── stage1_db.rs
        ├── stage2_browser_test.rs
        ├── stage3_search.rs
        ├── stage4_download.rs
        └── main.rs              # Das finale, zusammengeführte CLI (Clap)
```

---

### 4. Der Stufenweise Implementierungs- & Testplan

#### Stufe 1: Datenmodelle & Datenbank-Infrastruktur
*   **Aktion:** Erstellen von `00_models.rs`, `01_db_setup.rs` und `01b_db_repo.rs`.
*   **Fokus:** SQLite-Datei lokal anlegen, Tabellen erstellen, Dummy-Daten schreiben und lesen.
*   **Kompilieren mit:** `cargo run --bin stage1_db --features "db"`
*   **Ziel:** Du siehst eine `jobs_minutils.db` in deinem Ordner, die du mit einem DB-Browser öffnen kannst. Alles läuft pfeilschnell via `libsql`.

#### Stufe 2: Web-Automatisierung Basis ("Proof of Life")
*   **Aktion:** Erstellen von `02_web_core.rs`.
*   **Fokus:** `chromiumoxide` starten, Google aufrufen, Screenshot machen oder Seitentitel in der Konsole ausgeben.
*   **Kompilieren mit:** `cargo run --bin stage2_browser_test --features "scraper"`
*   **Ziel:** Sicherstellen, dass Headless-Chrome auf dem System sauber über die Rust-Async-Runtime angesteuert werden kann.

#### Stufe 3: Navigation & Suche auf Roche Careers
*   **Aktion:** Erstellen von `03_scraper_roche.rs`.
*   **Fokus:** Navigation zu Roche. Klick auf Cookie-Banner. Klick auf "Schweiz". Iteration durch die Paginierung, um rohe href-Links zu extrahieren. Noch **kein** Download der Job-Detailseiten! Nur die Liste der URLs (wie die alte `jobs.txt`).
*   **Kompilieren mit:** `cargo run --bin stage3_search --features "scraper"`
*   **Ziel:** Am Ende gibt das Programm eine Liste aller Schweiz-Job-URLs im Terminal aus.

#### Stufe 4: Async Download & JSON Extraktion (Trennung von DB)
*   **Aktion:** Erstellen von `04_downloader.rs` und `05_json_extractor.rs`.
*   **Fokus:** Die URLs aus Stufe 3 asynchron per HTTP GET (`reqwest`) laden. Aus dem HTML-String das JSON (`phApp.ddo`) mittels Regex extrahieren.
*   **Kompilieren mit:** `cargo run --bin stage4_download --features "scraper"`
*   **Ziel:** Speichern der extrahierten JSON-Strings als einfache `.json` Dateien im Ordner `jobs_html/` (genau wie im Python-Skript). **Wir schreiben hier bewusst noch nicht in die DB!** Wir wollen uns die extrahierten JSON-Dateien erst in Ruhe ansehen.

#### Stufe 5: Parsing & Datenbank-Ingestion
*   **Aktion:** Erstellen von `06_data_ingestion.rs`.
*   **Fokus:** Wir lesen die JSON-Dateien von der Festplatte. Wir nutzen `serde_json`, um sie in die Strukturen aus `00_models.rs` zu parsen. Dann nutzen wir `01b_db_repo.rs`, um sie in die SQLite zu schreiben.
*   **Test:** Dies kann nun kombiniert werden. Wir lesen JSON -> Wir prüfen Logausgaben -> Wir schreiben in die DB.
*   **Ziel:** Eine gefüllte, relationale SQLite-Datenbank.

#### Stufe 6: KI-Infrastruktur & Job-Annotation
*   **Aktion:** Erstellen von `07_ai_core.rs` (Traits) und `07b_ai_gemini.rs`.
*   **Fokus:** Implementierung der `rig` Library. Aus der DB werden ungelesene Jobs geladen (z.B. 10 Stück). Diese werden an Gemini gesendet, um das `job_summary` und die generelle `slide_tag_relevance` zu generieren.
*   **Kompilieren mit:** `cargo run --bin stage6_ai_test --features "db ai"`
*   **Ziel:** Update der SQLite mit den AI-Annotationen. Strikte Typsicherheit (die KI *muss* einen JSON-Array zurückgeben, andernfalls wirft Rust einen sauberen Fehler und versucht es erneut).

### Stufe 7: Historisiertes DB-Schema & Traits
*   **Dateien:** Erstelle `src/01c_db_traits.rs`, erweitere `src/00_models.rs` und `src/01_db_setup.rs`.
*   **Aufgabe:** 
    1. Definiere das `DatabaseProvider` Trait.
    2. Erweitere das SQLite Setup: 
       - Tabelle `candidates` (id INTEGER PRIMARY KEY, oauth_sub TEXT UNIQUE, name TEXT, profile_text TEXT).
       - Tabelle `candidate_matches` (id INTEGER PRIMARY KEY, candidate_id INTEGER, job_identifier TEXT, model_used TEXT, score INTEGER, explanation TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP).
       - Ändere die Job-Speicherung so ab, dass bei einem Update (z.B. geänderte Beschreibung) ein neuer Datensatz in einer `job_history` Tabelle angelegt wird oder ein `updated_at` Timestamp gesetzt wird.
*   **Test:** Schreibe `bin/stage7_db_v2.rs`, um Dummy-Kandidaten und mehrere Matches für den gleichen Job zu speichern. Lese danach mit `ORDER BY created_at DESC LIMIT 1` das aktuellste Match aus.

### Stufe 8: Scraper Politeness & In-Memory Pipeline
*   **Dateien:** Erweitere `04_downloader.rs` und erstelle `06_pipeline_orchestrator.rs`.
*   **Aufgabe:**
    1. Baue `polite_delay(20, 60)` (20-60 Sekunden Pause) in den Downloader ein, um den nächtlichen Lauf auf ~20 Minuten zu strecken.
    2. Verbinde den Downloader direkt mit dem JSON-Extractor, ohne Dateien auf die Festplatte zu schreiben.
    3. **Debug-Logik:** Wenn die Pipeline mit `debug = true` gestartet wird, speichere das extrahierte JSON zusätzlich unter `debug_dumps/YYYY-MM-DD/job_{identifier}.json`.
*   **Test:** `bin/stage8_polite_scrape.rs`.

### Stufe 9: Smart Batching & Token Limiting für Gemini
*   **Dateien:** Erstelle `07d_ai_rate_limiter.rs` und `07e_ai_batch_builder.rs`.
*   **Aufgabe:** 
    1. Implementiere Logik für die Gemini-Modelle aus der Konfigurationstabelle.
    2. Die Funktion nimmt eine Liste von `Job`s und ein `Candidate` Profil.
    3. Sie iteriert über die Jobs, zählt die Wörter des Prompts (`text.split_whitespace().count()`). Ein Wort = ~1.33 Token.
    4. Wenn das Limit von `tpm_limit * 0.8` (Sicherheitspuffer) erreicht ist, wird der Request gesendet.
    5. Implementiere einen Zähler, der `tokio::time::sleep` aufruft, wenn das `rpm` (Requests per Minute) Limit erreicht ist, bis die Minute abgelaufen ist.
*   **Test:** `bin/stage9_rate_limit_test.rs`.

### Stufe 10: Axum Web-Server & Authentifizierung
*   **Dateien:** Erstelle `11_web_server.rs` und `12_auth.rs`.
*   **Aufgabe:**
    1. Nutze `axum`, `tokio` und `tower-sessions`.
    2. Richte OAuth2 (GitHub oder Google) ein.
    3. Erstelle minimale Routen: `/login`, `/auth/callback`, `/logout`.
    4. Speichere den eingeloggten Nutzer über das `DatabaseProvider` Trait in der `candidates` Tabelle ab.
*   **Cargo Features:** Sicherstellen, dass dies nur kompiliert, wenn das `web` Feature aktiv ist.
*   **Test:** `bin/stage10_web.rs` (Sollte auf localhost:3000 lauschen und via Nginx unter `/app/` erreichbar sein).

### Stufe 11: Web-UI (Askama Templates)
*   **Dateien:** Erstelle Ordner `templates/`, erstelle `13_web_ui.rs`.
*   **Aufgabe:**
    1. Nutze Askama für serverseitiges HTML-Rendering.
    2. **Route `/app/profile`:** Ein Formular, in dem der User seinen Lebenslauf (Text) einkopieren kann.
    3. **Route `/app/dashboard`:** Zeigt eine Tabelle/Liste der neuesten Auswertungen (`candidate_matches` verknüpft mit `jobs`), gefiltert für die `candidate_id` der aktuellen Session.
    4. Baue einen "Re-Evaluate" Button, der einen asynchronen Task anwirft, um alle Jobs für diesen Kandidaten mit der KI neu zu bewerten (ideal, wenn das Profil geändert wurde).

### Stufe 12: Nightly Cron-Job
*   **Dateien:** Erstelle `14_scheduler.rs`.
*   **Aufgabe:**
    1. Nutze `tokio-cron-scheduler`.
    2. Richte einen Job ein, der jede Nacht um 02:00 Uhr triggert.
    3. Ablauf: `Scrape Jobs -> Finde Deltas (Neue/Geänderte Jobs) -> Speichere in DB -> Hole alle Kandidaten -> Generiere AI Matches (mit Rate Limiting aus Stufe 9) -> Speichere Ergebnisse in DB`.
*   **Test:** `bin/stage12_cron_test.rs` (Mit einem minütlichen Cron-Auslöser zum Testen).

### Stufe 13: Main CLI Zusammenführung
*   **Dateien:** Überarbeite `bin/main.rs`.
*   **Aufgabe:** Nutze `clap`, um das Programm steuerbar zu machen:
    *   `rs-scrape serve`: Startet Axum (Port 3000) UND den Background-Cron-Job.
    *   `rs-scrape trigger-scrape --debug`: Startet den Scraper sofort im Debug-Modus.
    *   `rs-scrape force-match --candidate-id <ID>`: Zwingt die KI, alle Jobs für einen spezifischen User sofort neu zu bewerten.
