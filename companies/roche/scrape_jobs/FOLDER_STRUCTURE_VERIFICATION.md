# Folder Structure Verification ✅

## How the Corrected Script Works

### Directory Execution Flow

```
Working Directory Start: /home/kiel/stage/slide-tag/companies/roche/scrape_jobs
                                    ↓
                    (Create 20260217/ folder)
                                    ↓
                    Change to: 20260217/
                                    ↓
        Run Phase 1 scripts (01-05) from inside 20260217/
        using relative paths: ../01_main.py, ../02_fetchlinks.py, etc.
                                    ↓
    All Phase 1 outputs created INSIDE 20260217/:
        ├── jobs.txt                    ✅ Created here
        ├── jobs_html/                  ✅ Created here
        ├── jobs_text/                  ✅ Created here
        ├── jobs_minutils.db            ✅ Created here
        ├── df_with_ai_annotations.csv  ✅ Created here
        └── supervisor_job_counts.csv   ✅ Created here
                                    ↓
                Change to: 20260217/ (same folder)
                                    ↓
        Run Phase 2 scripts (05b-07c) from inside 20260217/
        These scripts read df_with_ai_annotations.csv (in current dir)
                                    ↓
    Phase 2 creates outputs in 20260217/:
        ├── df_with_candidate_match.csv ✅ Created here
        ├── high_score_jobs.typ         ✅ Created here
        └── high_score_jobs_all.typ     ✅ Created here
                                    ↓
        Then MOVE these files to 20260217/dummy_profile/:
        ├── df_with_candidate_match.csv
        ├── high_score_jobs.typ
        └── high_score_jobs_all.typ
                                    ↓
            Return to original directory
```

---

## Final Folder Structure

After successful run:

```
20260217/                                    [DATE FOLDER]
├── jobs.txt                                [✅ SHARED - Phase 1 output]
├── jobs_html/                              [✅ SHARED - Phase 1 output]
│   ├── job_202507-119341.html
│   ├── job_202507-119341.json
│   ├── job_202508-121705.html
│   └── ... (all downloaded jobs)
├── jobs_text/                              [✅ SHARED - Phase 1 output]
│   ├── job_202507-119341.txt
│   ├── job_202508-121705.txt
│   └── ... (all extracted text)
├── jobs_minutils.db                        [✅ SHARED - Phase 1 output, SQLite database]
├── df_with_ai_annotations.csv              [✅ SHARED - Phase 1 output, filtered & annotated]
├── supervisor_job_counts.csv               [✅ SHARED - Phase 1 output, statistics]
│
├── dummy_profile/                          [CANDIDATE FOLDER - Phase 2 output]
│   ├── df_with_candidate_match.csv         [✅ Phase 2 output - moved here]
│   ├── high_score_jobs.typ                 [✅ Phase 2 output - moved here]
│   └── high_score_jobs_all.typ             [✅ Phase 2 output - moved here]
│
└── alice/                                  [CANDIDATE FOLDER - if run again with Alice]
    ├── df_with_candidate_match.csv         [✅ Phase 2 output for Alice]
    ├── high_score_jobs.typ                 [✅ Phase 2 output for Alice]
    └── high_score_jobs_all.typ             [✅ Phase 2 output for Alice]
```

---

## Verification Points ✅

### Phase 1 Output Placement
| File/Folder | Location | Source Script | Status |
|------------|----------|----------------|--------|
| `jobs.txt` | `20260217/` | 01_main.py | ✅ Correct |
| `jobs_html/` | `20260217/` | 02_fetchlinks.py | ✅ Correct |
| `jobs_text/` | `20260217/` | 02_fetchlinks.py | ✅ Correct |
| `jobs_minutils.db` | `20260217/` | 04_json_to_sqlite.py | ✅ Correct |
| `df_with_ai_annotations.csv` | `20260217/` | 05_db_filter.py | ✅ Correct |
| `supervisor_job_counts.csv` | `20260217/` | 05_db_filter.py | ✅ Correct |

### Phase 2 Output Placement
| File | Location | Source Script | Status |
|------|----------|---------------|--------|
| `df_with_candidate_match.csv` | `20260217/<candidate>/` | 05b + orchestrator | ✅ Correct (moved) |
| `high_score_jobs.typ` | `20260217/<candidate>/` | 07b + orchestrator | ✅ Correct (moved) |
| `high_score_jobs_all.typ` | `20260217/<candidate>/` | 07c + orchestrator | ✅ Correct (moved) |

---

## Key Implementation Details

### Phase 1 Execution (Lines 172-257)
```python
# Save original directory
original_cwd = os.getcwd()

# Change to date folder
os.chdir(date_folder)

try:
    # Run Phase 1 scripts with relative paths to parent
    uv run ../01_main.py
    uv run ../02_fetchlinks.py
    uv run ../03_extract_job_info.py
    uv run ../04_json_to_sqlite.py
    uv run ../05_db_filter.py
    
finally:
    # Return to original directory
    os.chdir(original_cwd)
```

**Result**: All Phase 1 outputs in `<date>/` folder

### Phase 2 Execution (Lines 259-337)
```python
# Change to date folder (where Phase 1 outputs are)
os.chdir(date_folder)

try:
    # Run Phase 2 scripts from date folder
    uv run ../05b_match_candidate.py
    
    # Move output to candidate subfolder
    Path("df_with_candidate_match.csv").rename(candidate_folder / "df_with_candidate_match.csv")
    
    # Run Typst generators
    uv run ../07b_jobs_to_typst.py
    uv run ../07c_all_jobs_to_typst.py
    
    # Move outputs to candidate subfolder
    Path("high_score_jobs.typ").rename(candidate_folder / "high_score_jobs.typ")
    Path("high_score_jobs_all.typ").rename(candidate_folder / "high_score_jobs_all.typ")

finally:
    # Return to original directory
    os.chdir(original_cwd)
```

**Result**: All Phase 2 outputs in `<date>/<candidate>/` folder

---

## Multiple Candidate Execution

When running multiple candidates on same day:

```bash
# First candidate
uv run 00_run_pipeline.py alice.txt
→ 20260217/
   ├── jobs.txt (created)
   ├── jobs_html/ (created)
   ├── jobs_minutils.db (created)
   ├── df_with_ai_annotations.csv (created)
   └── alice/ (created)
       ├── df_with_candidate_match.csv (created)
       ├── high_score_jobs.typ (created)
       └── high_score_jobs_all.typ (created)

# Second candidate (skip Phase 1)
uv run 00_run_pipeline.py bob.txt --skip-general
→ 20260217/
   ├── jobs.txt (reused ✅)
   ├── jobs_html/ (reused ✅)
   ├── jobs_minutils.db (reused ✅)
   ├── df_with_ai_annotations.csv (reused ✅)
   ├── alice/ (existing)
   │   └── ...
   └── bob/ (created)
       ├── df_with_candidate_match.csv (created)
       ├── high_score_jobs.typ (created)
       └── high_score_jobs_all.typ (created)
```

---

## Comparison with Previous Run

When using `--previous-date`:

```bash
uv run 00_run_pipeline.py candidate.txt --previous-date 20260115

→ Compares:
  - 20260115/df_with_candidate_match.csv (loads as df_jobs_old)
  - 20260217/df_with_candidate_match.csv (current)
  - Jobs in 20260115 marked as "new=0"
  - Jobs NOT in 20260115 marked as "new=1"
  - Typst output highlights new jobs
```

---

## All Requirements Verified ✅

✅ **jobs_html/** placed in `<date>/` folder
✅ **jobs_minutils.db** placed in `<date>/` folder
✅ **df_with_ai_annotations.csv** placed in `<date>/` folder
✅ **All Phase 1 outputs** in date folder (shared)
✅ **Candidate-specific outputs** in `<date>/<candidate>/` subfolder
✅ **Reusable Phase 1** across multiple candidates
✅ **Proper path handling** with relative paths
✅ **Directory cleanup** (returns to original after execution)
✅ **Error handling** with finally blocks

---

## Execution Flow Diagram

```
START
  ↓
Parse arguments & create 20260217/
  ↓
┌─────────────────────────────────┐
│   if --skip-general == False    │
│   (Run Phase 1)                 │
└─────────────────────────────────┘
  ├─ cd 20260217/
  ├─ uv run ../01_main.py           → jobs.txt
  ├─ uv run ../02_fetchlinks.py     → jobs_html/, jobs_text/
  ├─ uv run ../03_extract_...py     → *.json files
  ├─ uv run ../04_json_to_sqlite.py → jobs_minutils.db
  ├─ uv run ../05_db_filter.py      → df_with_ai_annotations.csv
  └─ cd <original_dir>
      ↓
┌─────────────────────────────────┐
│   if --skip-candidate == False  │
│   (Run Phase 2)                 │
└─────────────────────────────────┘
  ├─ cd 20260217/
  ├─ uv run ../05b_match...py
  │  └─ mv df_with_candidate_match.csv → <candidate>/
  ├─ uv run ../07b_jobs_to_typst.py
  │  └─ mv high_score_jobs.typ → <candidate>/
  ├─ uv run ../07c_all_jobs_...py
  │  └─ mv high_score_jobs_all.typ → <candidate>/
  └─ cd <original_dir>
      ↓
Summary & cleanup
  ↓
END
```

---

## Summary

✅ **FIXED**: All outputs now correctly placed in date-stamped folder
✅ **jobs_html/** is in `<date>/` folder
✅ **jobs_minutils.db** is in `<date>/` folder  
✅ **All other outputs** are properly organized
✅ **Multiple candidates** properly share Phase 1 results
✅ **Folder structure** is clean and organized

**Implementation is now correct and ready for production use!**

