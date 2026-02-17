# Implementation Summary: Roche Job Scraper Pipeline Streamlining

## Overview
Successfully implemented a comprehensive streamlining of the Roche Job Scraper & Analysis Pipeline with the following improvements:

1. **Master Orchestration Script** (`00_run_pipeline.py`) - Automates the entire workflow
2. **Command-line Arguments** - Added flexible parameter control to all AI-powered scripts
3. **Organized Output Structure** - Automatic date and candidate-based folder organization
4. **Token Management** - Configurable AI models and chunk sizes for different token budgets
5. **Updated Documentation** - Comprehensive README with examples and use cases

---

## Changes Made

### 1. **05b_match_candidate.py** - Candidate Profile as Argument
**Changes:**
- Added required positional argument: `candidate_profile_path`
- Added optional arguments:
  - `--model`: Choose AI model (default: gemini-3-flash-preview)
  - `--max-word-limit`: Max words per request (default: 5100)
  - `--separator`: Custom separator between job descriptions

**Usage:**
```bash
uv run 05b_match_candidate.py /path/to/candidate_profile.txt
uv run 05b_match_candidate.py /path/to/profile.md --model gemini-2.5-flash-lite --max-word-limit 4000
```

### 2. **05_db_filter.py** - AI Model and Chunk Size Configuration
**Changes:**
- Added optional arguments:
  - `--model`: Choose AI model (default: gemini-flash-latest)
  - `--max-len`: Max characters per API request (default: 200000)

**Usage:**
```bash
uv run 05_db_filter.py
uv run 05_db_filter.py --model gemini-2.5-flash-lite --max-len 150000
```

### 3. **07b_jobs_to_typst.py** - Previous Date Comparison
**Changes:**
- Added optional argument:
  - `--previous-date`: Date folder (YYYYMMDD) to mark new jobs

**Usage:**
```bash
uv run 07b_jobs_to_typst.py --previous-date 20260201
```

### 4. **07c_all_jobs_to_typst.py** - Previous Date Comparison
**Changes:**
- Added optional argument:
  - `--previous-date`: Date folder (YYYYMMDD) to mark new jobs

**Usage:**
```bash
uv run 07c_all_jobs_to_typst.py --previous-date 20260201
```

### 5. **00_run_pipeline.py** - NEW Master Orchestration Script
**Purpose:** Automates the complete workflow with flexible configuration

**Key Features:**
- Automatically creates dated folders (`YYYYMMDD/`)
- Organizes candidate-specific outputs in subfolders
- Runs Phase 1 (scripts 01-05) once and caches results
- Runs Phase 2 (scripts 05b-07c) for each candidate
- Reuses Phase 1 outputs across multiple candidates
- Supports dry-run mode for testing

**Arguments:**
```
Positional:
  candidate_profile_path          Path to candidate profile text file

Options:
  --candidate-name NAME           Name/identifier for candidate
  --skip-general                  Skip scripts 01-05 (general job collection)
  --skip-candidate                Skip scripts 05b-07c (candidate processing)
  --model MODEL                   Model for both 05 and 05b (default: auto-selected)
  --model-05 MODEL                Model specifically for script 05
  --model-05b MODEL               Model specifically for script 05b
  --max-len INT                   Max characters per request for 05 (default: 200000)
  --max-word-limit INT            Max words per request for 05b (default: 5100)
  --previous-date DATE            Previous run date (YYYYMMDD) for comparison
  --no-typst                      Skip Typst generation
  --dry-run                       Print commands without executing
```

**Usage Examples:**
```bash
# Single candidate run (creates 20260217/alice/ folder)
uv run 00_run_pipeline.py /path/to/candidate_alice.txt

# Multiple candidates (reuses 01-05 results)
uv run 00_run_pipeline.py /path/to/candidate_alice.txt
uv run 00_run_pipeline.py /path/to/candidate_bob.txt --skip-general

# With custom parameters
uv run 00_run_pipeline.py candidate.txt \
    --model gemini-2.5-flash-lite \
    --max-len 150000 \
    --max-word-limit 4000 \
    --previous-date 20260201

# Dry run (preview commands)
uv run 00_run_pipeline.py candidate.txt --dry-run

# Skip Typst generation
uv run 00_run_pipeline.py candidate.txt --no-typst

# Only collect jobs (skip candidate processing)
uv run 00_run_pipeline.py dummy.txt --skip-candidate
```

---

## Output Structure

Before this implementation, outputs were scattered and required manual folder management:
```
(root directory with mixed outputs)
```

Now, outputs are automatically organized:
```
<YYYYMMDD>/
├── jobs.txt                           # Shared: Job URLs
├── jobs_html/                         # Shared: Downloaded HTML files
├── jobs_text/                         # Shared: Text extraction from jobs
├── jobs_minutils.db                   # Shared: SQLite database
├── df_with_ai_annotations.csv         # Shared: Filtered & annotated jobs
├── supervisor_job_counts.csv          # Shared: Job statistics
│
└── <candidate_name>/
    ├── df_with_candidate_match.csv    # Candidate-specific: Matched jobs
    ├── high_score_jobs.typ            # Candidate-specific: Typst document
    └── high_score_jobs_all.typ        # Candidate-specific: All jobs Typst
```

---

## Workflow Examples

### Example 1: Single Candidate in One Command
```bash
uv run 00_run_pipeline.py ~/resume.txt --candidate-name alice
```
Creates: `20260217/alice/` with all outputs

### Example 2: Multiple Candidates, Token-Efficient Processing
```bash
# Run expensive Phase 1 once (collect and annotate jobs)
uv run 00_run_pipeline.py dummy.txt --skip-candidate

# Then run multiple candidates (cheap Phase 2 only, reuses Phase 1)
uv run 00_run_pipeline.py ~/alice.txt --skip-general
uv run 00_run_pipeline.py ~/bob.txt --skip-general
uv run 00_run_pipeline.py ~/charlie.txt --skip-general
```

### Example 3: Token Management with Different Models
```bash
# Phase 1 with cheaper, faster model
uv run 00_run_pipeline.py dummy.txt \
    --skip-candidate \
    --model-05 gemini-2.5-flash-lite \
    --max-len 150000

# Phase 2 with appropriate model for candidate matching
uv run 00_run_pipeline.py candidate.txt \
    --skip-general \
    --model-05b gemini-3-flash-preview \
    --max-word-limit 4000
```

### Example 4: New Job Comparison Across Dates
```bash
# Run with previous date reference
uv run 00_run_pipeline.py candidate.txt --previous-date 20260201

# This marks jobs not present in 20260201 as "new" in Typst output
# Can then highlight new job opportunities for the candidate
```

### Example 5: Dry Run for Validation
```bash
uv run 00_run_pipeline.py candidate.txt --dry-run

# Output preview:
# ================================================================================
# Step 01: Scraping job links from Roche careers website
# ================================================================================
# Command: uv run 01_main.py
# [DRY RUN] Command not executed
# ...
```

---

## Key Benefits

### For Workflow Automation
- **One-command pipeline**: Run entire workflow with a single command
- **Automatic date organization**: Folders created automatically by run date
- **Candidate isolation**: Each candidate's outputs in separate subfolder
- **Result caching**: Phase 1 results reused across candidates

### For Token Management
- **Model flexibility**: Different models for different phases
- **Adjustable chunk sizes**: Control `--max-len` and `--max-word-limit`
- **Staged processing**: Run expensive jobs collection once, then multiple candidates
- **Parameter inheritance**: Pass model/parameters through orchestrator to sub-scripts

### For Comparison & Tracking
- **New job highlighting**: Compare against previous runs to mark new jobs
- **Historical tracking**: Each date preserves previous results for comparison
- **Candidate tracking**: Easy to see which candidates have been processed

### For Development & Debugging
- **Dry run mode**: Test command sequence without executing
- **Skippable phases**: Run only specific phases as needed
- **Detailed logging**: Each script outputs progress information

---

## Migration Guide

### From Manual Execution
**Before:**
```bash
uv run 01_main.py
uv run 02_fetchlinks.py
for f in jobs_html/*.html; do uv run 03_extract_job_info.py "$f"; done
uv run 04_json_to_sqlite.py jobs_html/*.json
uv run 05_db_filter.py
mkdir candidate_alice
cd candidate_alice
uv run ../05b_match_candidate.py /path/to/alice.txt
uv run ../07b_jobs_to_typst.py
uv run ../07c_all_jobs_to_typst.py
cd ..
```

**After:**
```bash
uv run 00_run_pipeline.py /path/to/alice.txt
```

### For Multiple Candidates
**Before:** Repeat entire pipeline for each candidate, storing results manually

**After:**
```bash
# Expensive part runs once
uv run 00_run_pipeline.py dummy.txt --skip-candidate

# Cheap part runs for each candidate
uv run 00_run_pipeline.py alice.txt --skip-general
uv run 00_run_pipeline.py bob.txt --skip-general
```

---

## README Updates

The README has been completely updated with:
1. **New Quick Start section** with orchestration script examples
2. **Pipeline Architecture** explaining three-phase structure
3. **Streamlined Workflow** with multiple use case examples
4. **Output Structure** showing automatic organization
5. **Detailed Configuration** for each script's parameters
6. **Advanced Usage** section for token management and dry runs
7. **Manual Execution** section for legacy workflows
8. **Compilation** instructions for Typst

---

## Files Modified

| File | Changes |
|------|---------|
| `05_db_filter.py` | Added `--model` and `--max-len` arguments |
| `05b_match_candidate.py` | Added required `candidate_profile_path` + `--model`, `--max-word-limit`, `--separator` arguments |
| `07b_jobs_to_typst.py` | Added `--previous-date` argument, conditional date loading |
| `07c_all_jobs_to_typst.py` | Added `--previous-date` argument, conditional date loading |
| `README.md` | Comprehensive rewrite with new examples and documentation |

| File | Status |
|------|--------|
| `00_run_pipeline.py` | ✓ CREATED - New master orchestration script (290+ lines) |

---

## Testing Performed

✓ Python syntax validation on all modified files
✓ Argument parser testing for all scripts
✓ Help output verification
✓ Orchestration script structure validation
✓ No compilation errors

---

## Next Steps (Optional Enhancements)

1. **Integration tests**: Test with actual job data
2. **Logging configuration**: Add log file output alongside console
3. **Progress bar**: Add visual progress indicators for long-running phases
4. **Email notifications**: Send completion notifications
5. **Error recovery**: Checkpoint system for resuming failed runs
6. **Configuration file**: Support `.env` or config file instead of CLI args

---

## Support

All scripts include `--help` output:
```bash
uv run 00_run_pipeline.py --help
uv run 05b_match_candidate.py --help
uv run 05_db_filter.py --help
uv run 07b_jobs_to_typst.py --help
uv run 07c_all_jobs_to_typst.py --help
```

For detailed usage instructions, see the updated README.md

