Hier ist ein umfassender, stufenweiser Implementierungsplan für den Rust-Port der Roche Job Scraper Pipeline. 

Um deine Anforderungen zu erfüllen (isolierte Testbarkeit, minimale Abhängigkeiten, nummerierte Dateistruktur), wählen wir eine Architektur, die stark auf **Feature-Flags in Cargo** und **Modularisierung durch Traits** setzt.

---

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

#### Stufe 7: Erweitertes DB-Schema & Job-Historisierung
*   **Aktion:** Erstellen von `01c_db_schema_v2.rs` (oder Update von `01_db_setup.rs` / `01b_db_repo.rs`).
*   **Fokus:** Anpassung der Datenbank für die neuen Anforderungen:
    *   Tabelle `candidates` (id, oauth_id, name, profile_text, created_at).
    *   Tabelle `candidate_job_matches` (id, candidate_id, job_identifier, ai_model, score, explanation, created_at). So können wir problemlos mehrere Auswertungen speichern und via `ORDER BY created_at DESC LIMIT 1` immer die aktuellste laden.
    *   **Historisierung:** Anpassung der `jobs`-Tabelle oder Erstellung einer `job_history`-Tabelle, damit bei jedem nächtlichen Scraping Updates an Jobs (z.B. geänderte Deadlines) als neue Historien-Einträge gespeichert werden, anstatt sie hart zu überschreiben.
*   **Test:** Ein isoliertes Test-Skript `bin/stage7_db_v2.rs`, das einen Dummy-Kandidaten und mehrfache Match-Ergebnisse schreibt und das aktuellste abfragt.

#### Stufe 8: Smart Batching & Token-Management für AI
*   **Aktion:** Erstellen von `07d_ai_batching.rs` und Update von `07_ai_core.rs`.
*   **Fokus:** Implementierung des "Wort-Zählers". 
    *   Eine Funktion generiert für jeden Job/Kandidaten einen Sub-Prompt.
    *   Eine Schleife sammelt diese Sub-Prompts, zählt die Wörter (`text.split_whitespace().count()`).
    *   Sobald das Limit von z.B. 14.000 Wörtern erreicht ist, wird der Batch an Gemini gesendet, die JSON-Antwort verarbeitet und der nächste Batch gestartet.
*   **Ziel:** Maximale Ausnutzung des Gemini Free Tiers ohne Risiko von `HTTP 429` (Too Many Requests) oder `400` (Token Limit exceeded).

#### Stufe 9: In-Memory Pipeline & Debug-Dumps (Vorbereitung für Automatisierung)
*   **Aktion:** Überarbeitung von `06_data_ingestion.rs` zu `06_pipeline_orchestrator.rs`.
*   **Fokus:** Entfernen des Zwangs, HTML/JSON auf die Festplatte zu schreiben. Der Datenfluss ist nun: `Reqwest -> String -> Regex/JSON-Parser -> Struct -> DB`.
*   **Debug-Feature:** Einbau einer Logik: Wenn die Applikation mit einem Debug-Flag gestartet wird, wird ein Ordner `debug_dumps/YYYY-MM-DD_HH-MM/` erstellt und rohe HTML/JSON-Dateien (benannt nach der Job-ID) abgelegt.
*   **Test:** `bin/stage9_full_scrape.rs`, das einmal komplett im RAM läuft und nur am Ende in die SQLite schreibt.

#### Stufe 10: Web-Server Basis & OAuth Integration
*   **Aktion:** Erstellen von `11_web_server.rs` und `12_auth.rs`.
*   **Fokus:** Aufsetzen eines **Axum** Webservers. 
    *   Einrichtung von Session-Management (z.B. mit `tower-sessions`).
    *   Implementierung des OAuth-Flows (GitHub oder Google).
    *   Routen: `/` (Landing Page), `/login`, `/auth/callback`, `/dashboard`.
*   **Kompilieren mit:** `cargo run --bin stage10_web --features "web db"`
*   **Ziel:** Du kannst im Browser `localhost:3000` öffnen, dich via Google/Github einloggen und siehst danach deine OAuth-ID in der Konsole / im Browser.

#### Stufe 11: Web-UI: Profilverwaltung & Job-Matching Dashboard
*   **Aktion:** Erstellen von Askama-Templates (`templates/dashboard.html`, `templates/profile.html`) und Anbindung in `13_web_ui.rs`.
*   **Fokus:** 
    *   **Profil:** Ein Text-Feld im Browser, wo der eingeloggte Kandidat sein Profil/CV einkopieren kann. Speichern in der DB.
    *   **Dashboard:** Eine Ansicht, die die Liste der `candidate_job_matches` für den aktuellen User aus der Datenbank lädt (nur die neuesten pro Job). 
    *   UI-Elemente: Filter nach Score, Darstellung der historischen Entwicklung eines Jobs (Wann wurde er zuerst gepostet? Was hat sich geändert?).
*   **Ziel:** Eine voll funktionsfähige Web-App. Die AI hat im Hintergrund gearbeitet, der Nutzer konsumiert nur noch das Ergebnis.

#### Stufe 12: Nightly Scheduler (Cron-Job in Rust)
*   **Aktion:** Erstellen von `14_scheduler.rs`.
*   **Fokus:** Wir brauchen keinen externen Linux-Cronjob. Tokio kann das selbst. Ein asynchroner Task läuft im Hintergrund (`tokio::spawn`), prüft die Uhrzeit (z.B. mithilfe der Crate `tokio-cron-scheduler`) und startet jede Nacht um 03:00 Uhr die Scraper-Pipeline aus Stufe 9.
*   *Workflow Nachts:* Scrape Roche -> Lade DB Historie -> Identifiziere neue/geänderte Jobs -> Sende unbewertete Jobs an Gemini (Batching) -> Speichere in DB.
*   **Ziel:** Ein Zero-Maintenance System. Der Server läuft durchgängig.

#### Stufe 13: Das finale Binary (CLI + Server)
*   **Aktion:** Ausprogrammieren von `src/bin/main.rs`.
*   **Fokus:** Alles kommt zusammen in einem mächtigen CLI (mithilfe von `clap`).
    *   `rs-scrape serve` -> Startet Webserver & Nightly Scheduler.
    *   `rs-scrape scrape-now --debug-dump` -> Triggert einen sofortigen Scrape-Vorgang und speichert HTMLs zur Fehleranalyse.
    *   `rs-scrape evaluate-candidate <ID>` -> Erzwingt eine sofortige AI-Neubewertung für einen bestimmten Kandidaten (z.B. weil er sein Profil geändert hat).
