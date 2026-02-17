# Example Workflows

## Workflow 1: Process One Candidate

Simplest workflow - collect jobs and match to one candidate profile.

```bash
# Run the pipeline for Alice
uv run 00_run_pipeline.py ~/candidates/alice_profile.txt

# Outputs to: 20260217/alice/
# Time: ~30-45 minutes (first run includes job scraping)
```

---

## Workflow 2: Process Multiple Candidates (Same Day)

Run expensive job collection once, then process multiple candidates cheaply.

```bash
# Step 1: Collect and annotate all jobs (ONE TIME)
# This is the expensive step, takes ~30-45 minutes
uv run 00_run_pipeline.py /tmp/dummy.txt --skip-candidate

# Step 2: Match Alice to jobs (QUICK)
# This is fast, takes ~2-5 minutes
uv run 00_run_pipeline.py ~/candidates/alice_profile.txt --skip-general

# Step 3: Match Bob to jobs (QUICK)
uv run 00_run_pipeline.py ~/candidates/bob_profile.txt --skip-general

# Step 4: Match Charlie to jobs (QUICK)
uv run 00_run_pipeline.py ~/candidates/charlie_profile.txt --skip-general

# Outputs:
# 20260217/alice/high_score_jobs.typ
# 20260217/bob/high_score_jobs.typ
# 20260217/charlie/high_score_jobs.typ

# Time: ~35-50 minutes total (vs ~90-135 if done separately)
# Savings: ~40-55 minutes per additional candidate!
```

---

## Workflow 3: Token Management - Limited Budget

When API tokens are running low, use cheaper models and smaller chunks.

```bash
# Scenario: Running low on gemini-3-flash-preview tokens

# Step 1: Use cheaper model for general job collection
uv run 00_run_pipeline.py /tmp/dummy.txt \
    --skip-candidate \
    --model-05 gemini-2.5-flash-lite \
    --max-len 100000  # Smaller chunks = fewer tokens

# Step 2: Use budget wisely for candidate matching
uv run 00_run_pipeline.py ~/candidates/alice.txt \
    --skip-general \
    --model-05b gemini-2.5-flash-lite \  # Cheaper model
    --max-word-limit 3000              # Smaller chunks

# Result: Uses ~30-40% fewer tokens while still producing results
```

---

## Workflow 4: Monitor New Job Opportunities

Track which jobs are new compared to previous run, highlight them for candidates.

```bash
# Previous run was on 2026-01-15 in folder 20260115/
# Now it's 2026-02-17 and we want to see new jobs

# Run with comparison to previous date
uv run 00_run_pipeline.py ~/candidates/alice.txt \
    --previous-date 20260115

# Result:
# - Jobs in 20260115/ marked as "old" (new=0)
# - Jobs NEW since 20260115/ marked as "new" (new=1)
# - Typst document highlights new jobs for Alice
```

---

## Workflow 5: Debug and Dry Run

Test your command setup before actually running (useful for complex parameter sets).

```bash
# Want to test complex parameter configuration?
uv run 00_run_pipeline.py ~/candidates/alice.txt \
    --model gemini-2.5-flash-lite \
    --max-len 100000 \
    --max-word-limit 2000 \
    --previous-date 20260115 \
    --dry-run

# Output shows all commands that WOULD run:
# ================================================================================
#   Roche Job Scraper & Analysis Pipeline
# ================================================================================
# Date: 20260217
# Candidate: alice
# Output folder: 20260217/alice
# Candidate profile: /home/user/candidates/alice.txt
# Dry run: True
# ================================================================================
#
# ================================================================================
#   PHASE 1: General Job Collection (Scripts 01-05)
# ================================================================================
#
# ================================================================================
#   Step 01: Scraping job links from Roche careers website
# ================================================================================
# Command: uv run 01_main.py
# [DRY RUN] Command not executed
#
# ... (shows all other commands)
#
# Review the output, and if it looks good, remove --dry-run to execute
```

---

## Workflow 6: Custom Candidate Names

Process candidates with descriptive naming.

```bash
# Process candidate from file path
uv run 00_run_pipeline.py /downloads/resume_2026.pdf

# Default name extracted from filename: "resume_2026"
# Outputs to: 20260217/resume_2026/

# ✓ Better: Give it a meaningful name
uv run 00_run_pipeline.py /downloads/resume_2026.pdf \
    --candidate-name "alice_smith_ch"

# Outputs to: 20260217/alice_smith_ch/
# Much easier to identify!
```

---

## Workflow 7: Fast Re-Processing (No Typst)

Skip time-consuming Typst generation if you only need CSV data.

```bash
# Quick pass: Just get the matching data
uv run 00_run_pipeline.py ~/candidates/alice.txt --no-typst

# Result: Only creates df_with_candidate_match.csv
# No high_score_jobs.typ (saves ~1-2 minutes)

# Later, generate Typst manually if needed:
cd 20260217/alice/
uv run ../../07b_jobs_to_typst.py
uv run ../../07c_all_jobs_to_typst.py
```

---

## Workflow 8: Production Use - Full Day Processing

Processing multiple candidates throughout the day.

```bash
# 8:00 AM - Collect jobs (expensive, run once)
uv run 00_run_pipeline.py /tmp/dummy.txt --skip-candidate
# ~40 minutes

# 9:00 AM - Process first candidate batch
uv run 00_run_pipeline.py ~/candidates/alice.txt --skip-general   # 3 min
uv run 00_run_pipeline.py ~/candidates/bob.txt --skip-general     # 3 min
uv run 00_run_pipeline.py ~/candidates/charlie.txt --skip-general # 3 min

# 9:15 AM - Review outputs and compile to PDF
typst compile 20260217/alice/high_score_jobs.typ
typst compile 20260217/bob/high_score_jobs.typ
typst compile 20260217/charlie/high_score_jobs.typ

# 11:00 AM - Process second candidate batch (jobs still available)
uv run 00_run_pipeline.py ~/candidates/diana.txt --skip-general
uv run 00_run_pipeline.py ~/candidates/eve.txt --skip-general

# 2:00 PM - Evening batch
uv run 00_run_pipeline.py ~/candidates/frank.txt --skip-general
uv run 00_run_pipeline.py ~/candidates/grace.txt --skip-general

# Total: ~50 minutes actual computation, process 7 candidates!
```

---

## Workflow 9: Integration with External Systems

Automated processing pipeline suitable for cron jobs or CI/CD.

```bash
#!/bin/bash
# run_candidates.sh

set -e  # Exit on error

TIMESTAMP=$(date +%Y%m%d)
CANDIDATES_DIR="~/candidates"

echo "Starting job processing run at $TIMESTAMP"

# Collect jobs
echo "Phase 1: Collecting jobs..."
uv run 00_run_pipeline.py /tmp/dummy.txt --skip-candidate

# Process each candidate
for profile in $CANDIDATES_DIR/*.txt; do
    candidate_name=$(basename "$profile" .txt)
    echo "Phase 2: Processing $candidate_name..."
    
    uv run 00_run_pipeline.py "$profile" \
        --candidate-name "$candidate_name" \
        --skip-general \
        --previous-date "20260115"  # Compare to last month
done

# Compile results
echo "Compiling Typst documents..."
for typst_file in $TIMESTAMP/*/high_score_jobs.typ; do
    typst compile "$typst_file"
done

# Archive results
tar -czf "results_$TIMESTAMP.tar.gz" "$TIMESTAMP/"
echo "Pipeline complete! Results archived to results_$TIMESTAMP.tar.gz"

# Optionally: Send email notification, upload to server, etc.
```

Usage:
```bash
chmod +x run_candidates.sh
./run_candidates.sh

# Or in crontab (daily at 9 AM):
0 9 * * * /home/user/roche/scrape_jobs/run_candidates.sh
```

---

## Workflow 10: Comparison Study

Compare how different candidates score for the same job set.

```bash
# Setup: Process jobs once
uv run 00_run_pipeline.py /tmp/dummy.txt --skip-candidate

# Process multiple candidates for comparison
for candidate_file in ~/candidates/*.txt; do
    candidate_name=$(basename "$candidate_file" .txt)
    echo "Processing $candidate_name for comparison..."
    
    uv run 00_run_pipeline.py "$candidate_file" \
        --candidate-name "$candidate_name" \
        --skip-general
done

# Now you have for each candidate:
# - 20260217/alice/df_with_candidate_match.csv
# - 20260217/bob/df_with_candidate_match.csv
# - 20260217/charlie/df_with_candidate_match.csv

# You can now analyze:
# - Which jobs have highest average match score?
# - Which candidate is best fit for specific jobs?
# - Are there consensus strong/weak matches?

# Example Python analysis:
python3 << 'EOF'
import pandas as pd
import glob

# Load all candidate results
results = {}
for csv_file in glob.glob("20260217/*/df_with_candidate_match.csv"):
    candidate = csv_file.split("/")[1]
    results[candidate] = pd.read_csv(csv_file, index_col=0)

# Average match score per job
df_combined = pd.concat([
    df[["candidate_match_score"]].rename(columns={"candidate_match_score": candidate})
    for candidate, df in results.items()
], axis=1)

# Show best jobs for all candidates
avg_score = df_combined.mean(axis=1)
print(avg_score.nlargest(10))
EOF
```

---

## Quick Links

- **Full Documentation**: See README.md
- **Quick Reference**: See QUICK_REFERENCE.md
- **Implementation Details**: See IMPLEMENTATION_SUMMARY.md

