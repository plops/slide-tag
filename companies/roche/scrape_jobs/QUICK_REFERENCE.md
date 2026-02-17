# Quick Reference Guide

## Basic Usage

### Run pipeline for one candidate
```bash
uv run 00_run_pipeline.py /path/to/candidate_profile.txt
```

### Run for multiple candidates (reusing job collection)
```bash
# First run collects jobs (expensive)
uv run 00_run_pipeline.py dummy.txt --skip-candidate

# Then process each candidate (cheap, reuses results)
uv run 00_run_pipeline.py alice_profile.txt --skip-general
uv run 00_run_pipeline.py bob_profile.txt --skip-general
```

---

## Token-Saving Scenarios

### Low on tokens? Use cheaper models and smaller chunks
```bash
uv run 00_run_pipeline.py candidate.txt \
    --model gemini-2.5-flash-lite \
    --max-len 100000 \
    --max-word-limit 3000
```

### Use different models for different phases
```bash
# Cheap model for general job collection
uv run 00_run_pipeline.py dummy.txt \
    --skip-candidate \
    --model-05 gemini-2.5-flash-lite

# Better model for candidate matching
uv run 00_run_pipeline.py candidate.txt \
    --skip-general \
    --model-05b gemini-3-flash-preview
```

---

## Advanced Scenarios

### Test commands without executing (dry-run)
```bash
uv run 00_run_pipeline.py candidate.txt --dry-run
```

### Mark new jobs compared to previous date
```bash
uv run 00_run_pipeline.py candidate.txt --previous-date 20260201
```

### Skip Typst generation (faster, if only need CSV)
```bash
uv run 00_run_pipeline.py candidate.txt --no-typst
```

### Give candidate a custom name
```bash
uv run 00_run_pipeline.py /path/to/resume.pdf \
    --candidate-name "alice_smith"
```

---

## Output Locations

After running, outputs are in: `<YYYYMMDD>/<candidate_name>/`

Example (Feb 17, 2026, candidate "alice"):
```
20260217/alice/
├── df_with_candidate_match.csv    # Jobs matched to Alice
├── high_score_jobs.typ            # Candidate-matched jobs (Typst)
└── high_score_jobs_all.typ        # All jobs (Typst)
```

Shared outputs (reused across candidates):
```
20260217/
├── jobs_minutils.db
├── df_with_ai_annotations.csv
├── supervisor_job_counts.csv
└── jobs_html/
```

---

## Compile Typst to PDF

```bash
# Install typst if needed
# See: https://github.com/typst/typst

typst compile 20260217/alice/high_score_jobs.typ
typst compile 20260217/alice/high_score_jobs_all.typ
```

---

## Manual Execution (Individual Scripts)

If orchestration script is not needed:

```bash
# 1. Collect and process jobs (Phase 1)
uv run 01_main.py
uv run 02_fetchlinks.py
for f in jobs_html/*.html; do uv run 03_extract_job_info.py "$f"; done
uv run 04_json_to_sqlite.py jobs_html/*.json
uv run 05_db_filter.py

# 2. Match candidate (Phase 2)
uv run 05b_match_candidate.py /path/to/candidate.txt

# 3. Generate reports
uv run 07b_jobs_to_typst.py
uv run 07c_all_jobs_to_typst.py
```

---

## Get Help

```bash
# Orchestration script help
uv run 00_run_pipeline.py --help

# Individual script help
uv run 05b_match_candidate.py --help
uv run 05_db_filter.py --help
uv run 07b_jobs_to_typst.py --help
uv run 07c_all_jobs_to_typst.py --help
```

---

## Common Issues

### "candidate_profile_path is required"
Make sure you provide the path to the candidate profile file:
```bash
uv run 00_run_pipeline.py /path/to/your/profile.txt
```

### "GEMINI_API_KEY not set"
Export your API key:
```bash
export GEMINI_API_KEY="your_key_here"
uv run 00_run_pipeline.py candidate.txt
```

### "Previous date CSV not found"
If using `--previous-date`, ensure the date folder exists:
```bash
# This won't work if 20260201 doesn't exist
uv run 00_run_pipeline.py candidate.txt --previous-date 20260201

# Check what date folders exist
ls -d 202*
```

---

## Environment Setup (Once)

```bash
# Install dependencies
cd /path/to/scrape_jobs
uv sync

# Set API key (add to .bashrc or .zshrc for persistence)
export GEMINI_API_KEY="your_gemini_api_key_here"

# Verify setup
uv run 00_run_pipeline.py --help
```

