# Roche Job Scraper & Analysis Pipeline

This project is a multi-step pipeline designed to scrape job postings from the Roche careers website, process the data, enrich it using generative AI, and produce filtered, candidate-matched reports in various formats.

## Prerequisites

- Python 3.13+
- A Chrome or Chromium-based browser and the corresponding `chromedriver`.
- A `GEMINI_API_KEY` environment variable for AI-powered analysis steps.
- Candidate profile text files for the candidate matching step.

## Setup

1.  **Install Dependencies:**
    ```bash
    uv sync
    ```

2.  **Set Environment Variable:**
    Export your Gemini API key.
    ```bash
    export GEMINI_API_KEY="your_api_key_here"
    ```

3.  **Create Candidate Profiles:**
    Create text files with candidate resume/profile information. Examples:
    ```
    candidate_alice.txt
    candidate_bob.txt
    my_profile.md
    ```

## Pipeline Architecture

The pipeline consists of three phases:

### Phase 1: General Job Collection (Scripts 01-05)
Runs once per scraping cycle. Results are reused for all candidates.

| Step  | Script                      | Input(s)                                             | Output(s)                           | Description                                                                                             |
| :---- | :-------------------------- | :--------------------------------------------------- | :---------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **1** | `01_main.py`                | (Browser interaction)                                | `jobs.txt`                          | Launches a browser, navigates to the Roche careers site, filters for jobs in Switzerland, and saves job URLs. |
| **2** | `02_fetchlinks.py`            | `jobs.txt`                                           | `jobs_html/`, `jobs_text/`          | Asynchronously downloads the full HTML and extracts plain text for each job URL.                        |
| **3** | `03_extract_job_info.py`      | `jobs_html/*.html`                                   | `jobs_html/*.json`                  | Extracts a structured JSON data object embedded within each job's HTML file.                            |
| **4** | `04_json_to_sqlite.py`        | `jobs_html/*.json`                                   | `jobs_minutils.db`                  | Parses the JSON files and populates a normalized SQLite database with detailed job information.         |
| **5** | `05_db_filter.py`             | `jobs_minutils.db`                                   | `df_with_ai_annotations.csv`        | Filters jobs based on criteria and uses AI to generate a summary and a "Slide-tag relevance" score.     |

### Phase 2: Candidate-Specific Processing (Scripts 05b-07c)
Runs for each candidate. Outputs are organized in dated folders with candidate subfolders.

| Step  | Script                      | Input(s)                                             | Output(s)                           | Description                                                                                             |
| :---- | :-------------------------- | :--------------------------------------------------- | :---------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **5b**| `05b_match_candidate.py`      | `df_with_ai_annotations.csv`, `candidate_profile.txt`| `df_with_candidate_match.csv`       | Scores each job against the candidate profile using AI, creating a `candidate_match_score`.             |
| **7b**| `07b_jobs_to_typst.py`        | `df_with_candidate_match.csv`                        | `high_score_jobs.typ`               | Generates a Typst document for candidate-matched jobs with new job highlighting.                       |
| **7c**| `07c_all_jobs_to_typst.py`    | `df_with_candidate_match.csv`                        | `high_score_jobs_all.typ`           | Generates a Typst document with all jobs from the database.                                            |

### Optional Scripts

| Step  | Script                      | Input(s)                                             | Output(s)                           | Description                                                                                             |
| :---- | :-------------------------- | :--------------------------------------------------- | :---------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **6** | `06_jobs_to_markdown.py`      | `df_with_ai_annotations.csv`                         | `high_relevance_jobs.md`            | Generates a Markdown report for jobs with a high "Slide-tag relevance" score.                         |
| **7** | `07_jobs_to_latex.py`         | `df_with_candidate_match.csv`                        | `high_score_jobs.tex`               | Creates a professional LaTeX document of jobs with a high candidate match score, ready for PDF compilation. |

---

## Streamlined Workflow

### Quick Start: Orchestration Script

The **recommended approach** is to use the orchestration script which automates the entire workflow:

```bash
# Basic usage: process one candidate
uv run 00_run_pipeline.py /path/to/candidate_profile.txt

# Custom AI model and parameters
uv run 00_run_pipeline.py /path/to/candidate_alice.txt \\
    --model gemini-2.5-flash-lite \\
    --max-len 150000 \\
    --max-word-limit 4000

# Use previous date for new job comparison
uv run 00_run_pipeline.py /path/to/candidate_profile.txt \\
    --previous-date 20260201

# Give the candidate a custom name
uv run 00_run_pipeline.py /path/to/resume.txt --candidate-name alice

# Skip general collection if already done
uv run 00_run_pipeline.py /path/to/candidate_bob.txt --skip-general

# Skip candidate processing if only collecting jobs
uv run 00_run_pipeline.py /path/to/dummy.txt --skip-candidate
```

### Output Structure

The script creates a folder structure:
```
<YYYYMMDD>/
├── jobs_html/              # HTML files for all jobs (shared)
├── jobs_text/              # Text extraction for all jobs (shared)
├── jobs.txt                # Job URLs (shared)
├── jobs_minutils.db        # SQLite database (shared)
├── df_with_ai_annotations.csv  # Filtered & annotated jobs (shared)
├── supervisor_job_counts.csv   # Job statistics (shared)
└── <candidate_name>/       # Candidate-specific outputs
    ├── df_with_candidate_match.csv  # Jobs matched to candidate
    ├── high_score_jobs.typ          # Typst document (candidate matches)
    └── high_score_jobs_all.typ      # Typst document (all jobs)
```

### Multiple Candidates

To process multiple candidates in a single run (reusing 01-05 results):

```bash
# Run for first candidate
uv run 00_run_pipeline.py /path/to/candidate_alice.txt

# Run for second candidate (skips 01-05, reuses existing results)
uv run 00_run_pipeline.py /path/to/candidate_bob.txt --skip-general

# Run for third candidate
uv run 00_run_pipeline.py /path/to/candidate_charlie.txt --skip-general
```

---

## Detailed Configuration

### Script 05: General Job Annotation (`05_db_filter.py`)

Controls AI model and API request size:

```bash
uv run 05_db_filter.py \\
    --model gemini-flash-latest \\           # AI model to use
    --max-len 200000                        # Max characters per request
```

**Parameters:**
- `--model`: Choose AI model (e.g., `gemini-2.5-flash`, `gemini-flash-latest`)
- `--max-len`: Maximum character count per API request (larger = fewer requests but more risk of timeout)

### Script 05b: Candidate Matching (`05b_match_candidate.py`)

Requires candidate profile path and controls AI parameters:

```bash
uv run 05b_match_candidate.py /path/to/candidate_profile.txt \\
    --model gemini-3-flash-preview \\      # AI model to use
    --max-word-limit 5100 \\               # Max words per request
    --separator "\\n\\n---\\n\\n"          # Separator between job descriptions
```

**Parameters:**
- `CANDIDATE_PROFILE_PATH` (required): Path to candidate text file
- `--model`: Choose AI model (default: `gemini-3-flash-preview`)
- `--max-word-limit`: Maximum words per API request (default: 5100)
- `--separator`: String separator between job descriptions in batch

### Scripts 07b & 07c: Typst Generation

Generate Typst documents with optional new job highlighting:

```bash
uv run 07b_jobs_to_typst.py \\
    --previous-date 20260201               # Compare against previous date to mark new jobs

uv run 07c_all_jobs_to_typst.py \\
    --previous-date 20260201               # Mark jobs not present in previous date
```

**Parameters:**
- `--previous-date`: Date folder (YYYYMMDD) to compare against. Jobs not in that date are marked as "new"

---

## Manual Execution (Legacy)

If you prefer to run scripts individually:

```bash
# Phase 1: General job collection (run once)
uv run 01_main.py
uv run 02_fetchlinks.py
for f in jobs_html/*.html; do uv run 03_extract_job_info.py "$f"; done
uv run 04_json_to_sqlite.py jobs_html/*.json
uv run 05_db_filter.py

# Phase 2: Candidate-specific processing (run for each candidate)
mkdir -p 20260217/alice
cd 20260217/alice
uv run ../../05b_match_candidate.py /path/to/candidate_alice.txt
uv run ../../07b_jobs_to_typst.py --previous-date 20260201
uv run ../../07c_all_jobs_to_typst.py --previous-date 20260201
cd ../..

# Compile Typst to PDF
typst compile 20260217/alice/high_score_jobs.typ
```

---

## Advanced Usage

### Token Management

If you're running low on API tokens for a specific model, you can:

1. **Use a different model** in 05 (cheaper/faster):
   ```bash
   uv run 00_run_pipeline.py candidate.txt --model-05 gemini-flash-8b
   ```

2. **Reduce chunk sizes** to use fewer tokens per request:
   ```bash
   uv run 00_run_pipeline.py candidate.txt \\
       --max-len 100000 \\           # Smaller chunks for 05
       --max-word-limit 3000         # Smaller chunks for 05b
   ```

3. **Process in stages**:
   ```bash
   # Collect jobs (high cost)
   uv run 00_run_pipeline.py dummy.txt --skip-candidate
   
   # Process multiple candidates later (lower cost)
   uv run 00_run_pipeline.py alice.txt --skip-general
   uv run 00_run_pipeline.py bob.txt --skip-general
   ```

### Dry Run

Preview commands without executing:

```bash
uv run 00_run_pipeline.py candidate.txt --dry-run
```

---

## Compilation

To convert Typst documents to PDF:

```bash
# Install Typst if not already installed
# See https://github.com/typst/typst

typst compile <YYYYMMDD>/<candidate_name>/high_score_jobs.typ
typst compile <YYYYMMDD>/<candidate_name>/high_score_jobs_all.typ
```
