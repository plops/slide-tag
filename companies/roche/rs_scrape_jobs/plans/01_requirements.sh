#!/bin/bash
# rs_scrape_jobs/plans/01_requirements.sh

# This script collects the Python codebase and reference materials for porting to Rust.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche"
SCRAPE_JOBS_DIR="${ROOT_DIR}/scrape_jobs"

{
declare -a FILES=(
    "${SCRAPE_JOBS_DIR}/pyproject.toml"
    "${SCRAPE_JOBS_DIR}/README.md"
    "${SCRAPE_JOBS_DIR}/INDEX.md"
    "${SCRAPE_JOBS_DIR}/EXAMPLE_WORKFLOWS.md"
    "${SCRAPE_JOBS_DIR}/COMPLETION_SUMMARY.txt"
    "${SCRAPE_JOBS_DIR}/00_run_pipeline.py"
    "${SCRAPE_JOBS_DIR}/01_main.py"
    "${SCRAPE_JOBS_DIR}/02_fetchlinks.py"
    "${SCRAPE_JOBS_DIR}/03_extract_job_info.py"
    "${SCRAPE_JOBS_DIR}/04_json_to_sqlite.py"
    "${SCRAPE_JOBS_DIR}/05_db_filter.py"
    "${SCRAPE_JOBS_DIR}/05b_match_candidate.py"
    "${SCRAPE_JOBS_DIR}/07b_jobs_to_typst.py"
    "${SCRAPE_JOBS_DIR}/20260304/jobs_html/0001-Software-Tester-within-RMD-System-Development-contract.html"
    "${SCRAPE_JOBS_DIR}/20260304/jobs_html/0001-Software-Tester-within-RMD-System-Development-contract.json"
    "${SCRAPE_JOBS_DIR}/20260304/jobs_html/0002-Lab-Technician-contract.html"
    "${SCRAPE_JOBS_DIR}/20260304/jobs_html/0002-Lab-Technician-contract.json"
    "../docs/deps/libsql.md"
    "../docs/deps/rig.md"
    "../docs/deps/chromiumoxide.md"
    "../docs/deps/genai.md"
    "../docs/deps/llm-chain.md"
    "../docs/deps/clap.md"
    "../docs/deps/serde.md"
    "../docs/deps/tera.md"
    "../docs/deps/askama.md"
    "../docs/deps/typst-as-library.md"
    "../docs/deps/typst-core.md"
)

for i in "${FILES[@]}"; do
    if [ -f "$i" ]; then
        echo "// start of $i"
        cat "$i"
        echo "// end of $i"
    else
        echo "ERROR: File not found: $i" >&2
        exit 1
    fi
done

cat << 'EOF'

--- PROMPT ---

- Devise a specific implementation plan for porting the Python-based Roche Job Scraper Pipeline to Rust.
- Store the plan in `rs_scrape_jobs/plans/01b_rust_port_plan.md`.
- Focus on transitioning the current multi-script orchestration into a high-performance Rust CLI using an **Agentic Bridge Architecture**.

### Key Goals & Constraints:

1. **Unified CLI**: Subcommands for `collect`, `process`, `match`, and `report`.
2. **Scraper Engine**: Port `01_main.py`/`02_fetchlinks.py` using `chromiumoxide` for browser automation and async performance.
3. **Local Database Integration**: Use **Turso** (via the `libsql` crate) exclusively for handling the **local SQLite database file** asynchronously, providing high performance without needing a database server.
4. **Agentic "Inbox/Outbox" Workflow**:
   - Since some Enterprise AI accounts (Copilot, ChatGPT) may be UI-only, architect a file-based bridge.
   - Rust tool should aggregate tasks into `TODO_BATCH.md`.
   - External agents (Windsurf SWE-1.5, Copilot) process the batch and write to `RESPONSES.json`.
   - Rust uses `serde_json` to parse and apply these changes programmatically.
5. **AI Integration**: Design a modular "AI Provider" trait to allow swapping between different LLMs (Gemini, OpenAI, etc.) for job filtering and candidate matching.
6. **Reporting Engine**: Implement a report generator (Markdown/Typst) using a templating engine (like `tera` or `askama`) or a dedicated Typst crate.

### Architectural Phases to Plan:

- **Phase 1: Foundation & Scraping**: Setup `clap` CLI and `chromiumoxide` crawler. Define core `serde` structs for job data.
- **Phase 2: Data Persistence**: Implement `libsql`/local SQLite integration for high-performance job storage.
- **Phase 3: The AI Bridge**: 
  - Implement the `Rig`-based direct Gemini pipeline for immediate programmatic filtering.
  - Implement the "Inbox/Outbox" batcher for handling complex matching via external agents (Windsurf/Copilot).
  - Design a `Trait`-based provider system to swap between API-driven and File-driven matching.
- **Phase 4: Unified Reporting**: Port `07b_jobs_to_typst.py` to a Rust-native template engine (e.g., `tera`).

### Reference Materials:
- Port Python logic (XPaths, prompts) while respecting Rust's strict typing.
- Use `Rig` to guarantee that AI-annotated fields (relevance, summary) conform to the local database schema.
- Leverage `llm-chain` for the candidate matching phase to process multiple jobs in a single large prompt context.

EOF
} | xclip -selection clipboard

echo "Codebase and updated Agentic Rust porting prompt (local-first) have been copied to the clipboard."
