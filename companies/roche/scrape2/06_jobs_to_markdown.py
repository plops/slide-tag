import pandas as pd
import ast
import json

# read in the dataframe from previous step
try:
    df_slide = pd.read_csv('df_slide_with_ai_annotations.csv')
except Exception as e:
    print(f"Failed to read jobs_with_slide_tags.csv: {e}")
    df_slide = None


# Add: render selected jobs as markdown (exclude description and company_name)
def jobs_to_markdown(df: pd.DataFrame, min_relevance: int = 4, out_path: str | None = None):
    """
    Print jobs with slide_tag_relevance >= min_relevance as markdown.
    Excludes 'description' and 'company_name'. Writes to out_path if provided.
    """
    if df is None:
        print("jobs_to_markdown: no dataframe provided")
        return

    lines = []
    # ensure relevance can be compared robustly
    def relevance_of(x):
        try:
            return int(x)
        except Exception:
            try:
                return int(float(x))
            except Exception:
                return None

    # make sorting robust by converting relevance to numeric first
    dfc = df.copy()
    dfc['_slide_tag_relevance_num'] = pd.to_numeric(dfc.get('slide_tag_relevance'), errors='coerce')
    # sort by numeric relevance desc, keep original ordering for ties
    for _, row in dfc.sort_values(by='_slide_tag_relevance_num', ascending=False).iterrows():
        rel = relevance_of(row.get('slide_tag_relevance'))
        if rel is None or rel < min_relevance:
            continue

        job_id = row.get('job_id', '')
        title = row.get('title', '')
        # common useful fields (omit description & company_name)
        metadata = {
            "Date posted": row.get('date_posted'),
            "Apply URL": row.get('apply_url'),
            "Salary min": row.get('salary_min'),
            "Salary max": row.get('salary_max'),
            "Worker type": row.get('worker_type'),
            "Sub category": row.get('sub_category'),
            "Job profile": row.get('job_profile'),
            "Supervisory organization": row.get('supervisory_organization'),
            "Target hire date": row.get('target_hire_date'),
            "Openings": row.get('openings'),
            "Grade profile": row.get('grade_profile'),
            "Recruiting start date": row.get('recruiting_start_date'),
            "Job level": row.get('job_level'),
            "Grade": row.get('grade'),
            "Job family": row.get('job_family'),
            "Is evergreen": row.get('is_evergreen'),
            "Slide-tag relevance": rel,
        }

        lines.append(f"## {title}  \n**Job ID:** {job_id}\n")
        # metadata block
        for k, v in metadata.items():
            if pd.isna(v) or v in (None, ''):
                continue
            # format Apply URL as link
            if k == "Apply URL":
                lines.append(f"- **{k}:** [{v}]({v})")
            else:
                lines.append(f"- **{k}:** {v}")
        lines.append("")  # blank line

        # job_summary: may be list[str], string, or NaN
        js = row.get('job_summary')

        # Helper: try to coerce string representations of lists into actual lists
        def try_parse_list(value):
            if isinstance(value, list):
                return value
            if not isinstance(value, str):
                return None
            text = value.strip()
            if not text:
                return None
            # try JSON
            try:
                parsed = json.loads(text)
                if isinstance(parsed, list):
                    return parsed
            except Exception:
                pass
            # try python literal (e.g. "['a','b']")
            try:
                parsed = ast.literal_eval(text)
                if isinstance(parsed, list):
                    return parsed
            except Exception:
                pass
            return None

        parsed_list = try_parse_list(js)
        if parsed_list:
            lines.append("### Summary")
            for s in parsed_list:
                if s is None:
                    continue
                s = str(s).strip()
                if s:
                    lines.append(f"- {s}")
        elif isinstance(js, str) and js.strip():
            lines.append("### Summary")
            # try to split into lines if it's a block
            for s in js.splitlines():
                s = s.strip()
                if s:
                    lines.append(f"- {s}")
        # final spacer
        lines.append("\n---\n")

    out_text = "\n".join(lines).strip()
    if not out_text:
        print(f"No jobs found with slide_tag_relevance >= {min_relevance}")
        return

    # print to stdout
    print(out_text)

    # optionally save to file
    if out_path:
        try:
            with open(out_path, "w", encoding="utf-8") as f:
                f.write(out_text + "\n")
            print(f"Markdown written to {out_path}")
        except Exception as e:
            print(f"Failed to write markdown to {out_path}: {e}")

# After AI annotations and CSV export, produce markdown for high-relevance jobs
if 'df_slide' in globals():
    try:
        jobs_to_markdown(df_slide, min_relevance=4, out_path='high_relevance_jobs.md')
    except Exception as e:
        print(f"Failed to produce markdown: {e}")
