
### `rs_scrape_jobs/plans/01b_rust_port_plan.md`

```markdown
# Implementation Plan: Rust Port of Roche Job Scraper Pipeline

## 1. Architectural Overview

The objective is to transition from a multi-script Python pipeline into a single, highly concurrent Rust binary (`rs-scrape`). This architecture will leverage asynchronous I/O (`tokio`), headless browser automation (`chromiumoxide`), local high-performance SQLite (`libsql`), and a dual-mode AI integration system (API-driven via `rig`/`llm-chain` or File-driven via an Agentic Bridge).

### Core Technology Stack
*   **CLI Orchestration:** `clap` (Subcommand-based router)
*   **Web Scraping:** `chromiumoxide` (CDP-based headless scraping)
*   **Concurrency & HTTP:** `tokio`, `reqwest` (for rapid async fetching)
*   **Data Persistence:** `libsql` (Local asynchronous SQLite)
*   **Data Serialization:** `serde`, `serde_json`
*   **AI Integration:** `rig` (typed LLM extraction), `llm-chain` (prompt batching)
*   **Reporting:** `askama`/`tera` (Templating), `typst` & `typst-pdf` (Native PDF generation)

---

## 2. Phase-by-Phase Implementation Plan

### Phase 1: Foundation & Scraping Engine (Replaces `01_main.py` & `02_fetchlinks.py`)

**Goal:** Establish the CLI skeleton, core data structures, and the asynchronous browser automation.

1.  **CLI Setup (`clap`):**
    *   Define `Cli` struct with subcommands: `collect`, `process`, `match`, `report`.
    *   Implement global flags (e.g., `--verbose`, `--dry-run`).
2.  **Core Data Models (`serde`):**
    *   Define `Job`, `Location`, `Skill` structs mapping precisely to the embedded JSON schema (`phApp.ddo`) found in the Roche HTML.
3.  **Headless Crawler (`chromiumoxide`):**
    *   Initialize the browser instance inside `tokio::spawn` to handle the DevTools protocol stream.
    *   Navigate to the Roche careers site, handle cookie banners, and interact with the "Schweiz" filter.
    *   Implement a robust pagination loop to extract all `<a>` tags with `data-ph-at-id='job-link'`.
4.  **Async Downloader:**
    *   Use `reqwest` streams (or parallel `chromiumoxide` pages) to download the HTML for all collected URLs concurrently.
    *   Extract the `phApp.ddo` JSON blob using regex or basic string parsing, bypassing the need for heavy DOM parsing like `BeautifulSoup`.

### Phase 2: Data Persistence (Replaces `03_extract_job_info.py` & `04_json_to_sqlite.py`)

**Goal:** Store normalized job data into a local SQLite database asynchronously.

1.  **Local Database Initialization (`libsql`):**
    *   Use `libsql::Builder::new_local("jobs.db").build().await`.
    *   Execute `CREATE TABLE IF NOT EXISTS` for `Jobs`, `Locations`, `Skills`, `Job_Locations`, and `Job_Skills`.
2.  **Data Ingestion Pipeline:**
    *   Parse the extracted JSON blobs using `serde_json`.
    *   Implement `UPSERT` logic using prepared statements (`conn.prepare(...)`).
    *   Use transactional batching to insert hundreds of jobs and relationships concurrently without locking the DB file.

### Phase 3: The AI Bridge (Replaces `05_db_filter.py` & `05b_match_candidate.py`)

**Goal:** Implement a modular AI pipeline with a Trait-based provider system to filter jobs and score candidates.

1.  **Provider Trait Design:**
    ```rust
    #[async_trait]
    pub trait AiProvider {
        async fn annotate_jobs(&self, jobs: Vec<Job>) -> Result<Vec<JobAnnotation>>;
        async fn match_candidate(&self, profile: &str, jobs: Vec<Job>) -> Result<Vec<CandidateMatch>>;
    }
    ```
2.  **Direct API Provider (`rig` & `llm-chain`):**
    *   Implement `GeminiProvider`.
    *   Use `llm-chain`'s map-reduce capabilities to chunk job descriptions into ~20k token blocks.
    *   Use `rig`'s `Extractor` with `schemars` to guarantee the LLM returns an exact JSON array matching the Rust `CandidateMatch` struct.
3.  **Agentic Bridge Provider (Inbox/Outbox Workflow):**
    *   *Outbox Phase:* Query the DB for unprocessed jobs. Serialize the context and formatting instructions into a Markdown file (`TODO_BATCH.md`). Include the candidate's profile.
    *   *Pause State:* The CLI exits or waits, prompting the user: *"Agent context written to TODO_BATCH.md. Awaiting RESPONSES.json..."*
    *   *Inbox Phase:* Once the external agent (Copilot/Windsurf) generates `RESPONSES.json`, the CLI resumes via `rs-scrape match --resume`.
    *   *Reconciliation:* `serde_json` parses `RESPONSES.json`, validates the schema, and updates `candidate_match_score` and `job_summary` in the `libsql` database.

### Phase 4: Unified Native Reporting (Replaces `07b_jobs_to_typst.py` & `07c_all_jobs_to_typst.py`)

**Goal:** Generate dynamic Typst markup and compile it directly to PDF using Rust, avoiding external shell commands.

1.  **Type-Safe Templating (`askama`):**
    *   Create an Askama template `report.typ`.
    *   *Crucial Constraint:* Typst uses `{` and `}` heavily. Askama must be configured with custom delimiters (e.g., `[[` and `]]`) in `askama.toml` to avoid parsing conflicts with native Typst syntax.
    *   Pass the sorted `Vec<JobMatch>` from SQLite to the compiled template.
2.  **In-Memory Typst Compilation (`typst-as-library` & `typst-pdf`):**
    *   Implement the `typst::World` trait wrapper.
    *   Feed the Askama-rendered Typst string directly into `typst::compile`.
    *   Export the resulting `PagedDocument` directly to a `.pdf` file on disk using `typst_pdf::pdf`.
    *   *Benefit:* 100% standalone execution; no need for users to install the `typst` CLI globally.

---

## 3. Command Line Interface Design

The unified interface will look like this:

```bash
# Phase 1 & 2: Scrape and store
rs-scrape collect --output jobs.db

# Phase 3a: Direct programmatic annotation
rs-scrape process --db jobs.db --provider gemini --model gemini-1.5-flash

# Phase 3b: Agentic candidate matching
rs-scrape match --db jobs.db --profile alice.md --provider agentic --generate-batch TODO_BATCH.md
# ... (Agent processes batch) ...
rs-scrape match --db jobs.db --provider agentic --ingest RESPONSES.json

# Phase 4: Report generation
rs-scrape report --db jobs.db --threshold 4 --output alice_matches.pdf
```

## 4. Error Handling & Validation Strategy

*   **Custom Error Enums:** Use `thiserror` for unified error handling across DB, Network, and AI layers.
*   **Schema Enforcement:** LLM outputs via `rig` will strictly fail/retry if the extracted `slide_tag_relevance` is not an integer between 1 and 5.
*   **Orphaned State Recovery:** The `libsql` DB acts as the source of truth. If the Agentic Bridge fails or `RESPONSES.json` is malformed, the database remains uncorrupted and the batch can simply be regenerated.

## 5. Directory Structure for the Rust Project

```text
rs_scrape_jobs/
├── Cargo.toml
├── src/
│   ├── main.rs            # CLI entrypoint (clap setup)
│   ├── scraper.rs         # chromiumoxide and reqwest logic
│   ├── db.rs              # libsql schemas and queries
│   ├── models.rs          # Serde data structures
│   ├── ai/
│   │   ├── mod.rs         # AiProvider trait
│   │   ├── gemini.rs      # rig and llm-chain implementation
│   │   └── agentic.rs     # Inbox/Outbox file bridge logic
│   └── report/
│       ├── mod.rs         # typst-as-library compilation
│       └── templates.rs   # askama structs
└── templates/
    └── matches.typ        # Askama Typst template
```
.
