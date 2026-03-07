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

#### Stufe 7: Kandidaten-Matching (Agentic vs. API)
*   **Aktion:** Erstellen von `08_matcher.rs` und `07c_ai_agentic.rs`.
*   **Fokus:** Implementierung der Logik für Phase 2 der Pipeline. 
    *   *Weg A:* Über Gemini API direkt scoren.
    *   *Weg B (Inbox/Outbox):* Jobs und Kandidatenprofil als `TODO_BATCH.md` auf die Platte schreiben, warten, `RESPONSES.json` lesen und DB updaten.
*   **Ziel:** Die Datenbank enthält nun auch den `candidate_match_score` für einen spezifischen Kandidaten.

#### Stufe 8: Reporting & Typst-Generierung
*   **Aktion:** Erstellen von `09_templating.rs` (Askama) und `10_typst_compiler.rs`.
*   **Fokus:** Wir lesen Jobs mit Score >= 4 aus der DB. Wir übergeben die Typ-sicheren Structs an Askama, welches einen Typst-String erzeugt. Dieser wird direkt "in-memory" über `typst-as-library` zu einem PDF kompiliert.
*   **Kompilieren mit:** `cargo run --bin stage8_report --features "db pdf"`
*   **Ziel:** Es fällt ein fertiges `.pdf` Dokument heraus. Keine Zwischenschritte über die Typst-Kommandozeile mehr nötig.

#### Stufe 9: Das finale CLI (Zusammenführung)
*   **Aktion:** Ausprogrammieren von `src/bin/main.rs`.
*   **Fokus:** Integration von `clap`. Alle Module werden nun hinter Subcommands (`collect`, `process`, `match`, `report`) orchestriert. Hier werden auch alle Cargo-Features beim Bauen (`cargo build --release --all-features`) verlinkt.
*   **Ziel:** Ein einzelnes Binary (`rs-scrape`), das in der Lage ist, die gesamte Pipeline hochperformant auszuführen.

---

### Warum dieser Plan so gut funktioniert:

1.  **Sicherheit durch Isolation:** Du bleibst nicht in einem riesigen Monolithen stecken. Wenn das Scraping bricht, kannst du an der KI-Schnittstelle weiterarbeiten, da die JSONs auf der Platte (Stufe 4) oder in der SQLite (Stufe 5) zwischengespeichert sind.
2.  **Kompilierzeiten:** Wenn du am CLI oder der Datenbank schraubst, machst du das ohne das `pdf` oder `scraper` Feature. Die Build-Zeit liegt dann bei wenigen Sekunden statt einer Minute.
3.  **Klare Dateigrenzen:** Wenn die `03_scraper_roche.rs` zu lang wird (z.B. durch komplexe DOM-Abfragen), splitest du sie einfach in `03a_scraper_auth.rs` und `03b_scraper_search.rs`. Durch die Nummern bleibt die inhaltliche Abfolge sofort erkennbar.