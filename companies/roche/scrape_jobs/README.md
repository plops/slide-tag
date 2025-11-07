# Roche Job Scraper & Analysis Pipeline

This project is a multi-step pipeline designed to scrape job postings from the Roche careers website, process the data, enrich it using generative AI, and produce filtered, candidate-matched reports in various formats.

## Prerequisites

- Python 3.13+
- A Chrome or Chromium-based browser and the corresponding `chromedriver`.
- A `GEMINI_API_KEY` environment variable for AI-powered analysis steps.
- A `candidate_profile.txt` file for the candidate matching step.

## Setup

1.  **Install Dependencies:**
    ```bash
    uv sync
    source .venv/bin/activate
    ```

2.  **Set Environment Variable:**
    Export your Gemini API key.
    ```bash
    export GEMINI_API_KEY="your_api_key_here"
    ```

3.  **Create Candidate Profile:**
    Create a file named `candidate_profile.txt` in this directory and fill it with the resume or profile of the candidate you want to match against job descriptions.

## Pipeline Workflow

The pipeline is composed of several numbered Python scripts that should be run in order.

| Step  | Script                      | Input(s)                                             | Output(s)                           | Description                                                                                             |
| :---- | :-------------------------- | :--------------------------------------------------- | :---------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **1** | `01_main.py`                | (Browser interaction)                                | `jobs.txt`                          | Launches a browser, navigates to the Roche careers site, filters for jobs in Switzerland, and saves job URLs. |
| **2** | `02_fetchlinks.py`            | `jobs.txt`                                           | `jobs_html/`, `jobs_text/`          | Asynchronously downloads the full HTML and extracts plain text for each job URL.                        |
| **3** | `03_extract_job_info.py`      | `jobs_html/*.html`                                   | `jobs_html/*.json`                  | Extracts a structured JSON data object embedded within each job's HTML file.                            |
| **4** | `04_json_to_sqlite.py`        | `jobs_html/*.json`                                   | `jobs_minutils.db`                  | Parses the JSON files and populates a normalized SQLite database with detailed job information.         |
| **5** | `05_db_filter.py`             | `jobs_minutils.db`                                   | `df_with_ai_annotations.csv`        | Filters jobs based on criteria and uses AI to generate a summary and a "Slide-tag relevance" score.     |
| **5b**| `05b_match_candidate.py`      | `df_with_ai_annotations.csv`, `candidate_profile.txt`| `df_with_candidate_match.csv`       | Scores each job against the candidate profile using AI, creating a `candidate_match_score`.             |
| **6** | `06_jobs_to_markdown.py`      | `df_with_ai_annotations.csv`                         | `high_relevance_jobs.md`            | Generates a Markdown report for jobs with a high "Slide-tag relevance" score.                         |
| **7** | `07_jobs_to_latex.py`         | `df_with_candidate_match.csv`                        | `high_score_jobs.tex`               | Creates a professional LaTeX document of jobs with a high candidate match score, ready for PDF compilation. |

---

### How to Run

Execute the scripts in numerical order. For scripts that accept file arguments, you can use shell globbing.

```bash
# 1. Scrape job links
python 01_main.py

# 2. Download job pages
python 02_fetchlinks.py

# 3. Extract JSON from HTML
# (This script takes one file at a time, so a loop is needed)
for f in jobs_html/*.html; do python 03_extract_job_info.py "$f"; done

# 4. Populate the database from JSON files
python 04_json_to_sqlite.py jobs_html/*.json

# 5. Filter and annotate jobs with AI
python 05_db_filter.py

# 5b. Match jobs to a candidate profile
python 05b_match_candidate.py

# 6. Generate Markdown report (optional)
python 06_jobs_to_markdown.py

# 7. Generate LaTeX report
python 07_jobs_to_latex.py

# Compile the LaTeX report to PDF (requires a TeX distribution like TeX Live)
pdflatex high_score_jobs.tex
```
