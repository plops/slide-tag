export GEMINI_API_KEY=`cat ~/api_nano.txt`
#uv run ../01_main.py

# 2. Download job pages
#uv run ../02_fetchlinks.py

# 3. Extract JSON from HTML
# (This script takes one file at a time, so a loop is needed)
#for f in jobs_html/*.html; do uv run ../03_extract_job_info.py "$f"; done

# 4. Populate the database from JSON files
#uv run ../04_json_to_sqlite.py jobs_html/*.json

# 5. Filter and annotate jobs with AI
uv run ../05_db_filter.py

# 5b. Match jobs to a candidate profile
uv run ../05b_match_candidate.py

# 6. Generate Markdown report (optional)
uv run ../06_jobs_to_markdown.py

# 7. Generate LaTeX report
# uv run 07_jobs_to_latex.py
uv run ../07c_all_jobs_to_typst.py

typst compile high_score_jobs.typ
typst compile high_score_jobs_all.typ
