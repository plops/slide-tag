import pandas as pd
import ast
import json
import re

# --- Helper function to escape special Typst characters ---
def escape_typst(text: str) -> str:
    """
    Escapes special Typst characters in a given string.
    """
    if not isinstance(text, str):
        text = str(text)

    # Typst uses backslash to escape, so escape backslash first.
    # Then escape characters that have special meaning in Typst markup.
    # When text is enclosed in `[]` content blocks, fewer characters need escaping,
    # but for robustness against potential markup, it's safer to escape these.
    replacements = {
        '\\': r"\\",  # Must be first to prevent double-escaping other replacements
        '[': r"\[",
        ']': r"\]",
        '*': r"\*",
        '_': r"\_",
        '#': r"\#",
        '$': r"\$",
        '@': r"\@",
    }

    # Create a regex pattern that matches any of the keys.
    # `re.escape` ensures that special regex characters in the keys are treated literally.
    regex = re.compile('|'.join(re.escape(key) for key in replacements.keys()))
    return regex.sub(lambda match: replacements[match.group(0)], text)

# --- Typst Preamble Definition ---
# This sets up global document styles and properties.
TYPST_PREAMBLE = r"""
// Set global text font and size
#set text(font: "Fira Sans", size: 11pt)
// Set page margins and paper size
#set page(margin: (top: 1in, bottom: 1in, left: 1in, right: 1in), paper: "a4")
// Disable numbering for headings (sections and subsections)
#set heading(numbering: none)
// Set the color for links
// #set link(stroke: blue)
// Configure bullet lists
//#set enum(indent: 0.5em, body_indent: 0.5em, marker: "-")

// Global settings for tables to mimic a clean, booktabs-like style
#set table(
  // Default columns: first column auto-width (good for labels), second takes remaining space
  columns: (auto, 1fr),
  // Padding inside cells
  inset: 5pt,
  // Stroke configuration: thin horizontal lines only, no vertical lines
  stroke: (y: 0.5pt, x: none),
  // Align content to the start (left) of cells
  align: start,
)

// Document metadata for PDF properties
#show: doc => {
  set document(title: "Job Descriptions", author: "Automated Script", keywords: ("job", "description", "report")) // , lang: "en")
  doc // Render the document content
}
"""

# --- Typst Postamble (End of Document) ---
# Typst does not require an explicit command to end the document.
TYPST_POSTAMBLE = ""

def jobs_to_typst(
        df: pd.DataFrame, min_candidate_score: int = 4, out_path: str | None = None
):
    """
    Generate a Typst document for jobs with candidate_match_score >= min_candidate_score.
    Writes the output to out_path if provided.
    """
    if df is None:
        print("jobs_to_typst: no dataframe provided")
        return

    typst_job_blocks = []

    # --- Sort dataframe by candidate match score ---
    dfc = df.copy()
    dfc["candidate_match_score"] = pd.to_numeric(
        dfc.get("candidate_match_score"), errors="coerce"
    )
    df_sorted = dfc.sort_values(
        by="candidate_match_score", ascending=False
    )
    # --- End of sorting logic ---

    for _, row in df_sorted.iterrows():
        score = row.get("candidate_match_score")
        if pd.isna(score) or score < min_candidate_score:
            continue

        job_id = row.get("job_id", "")
        new = row.get("new", 0)
        title = row.get("title", "")
        apply_url = row.get("apply_url", "")

        # --- MODIFICATION START ---
        # If the URL ends with /apply, remove it
        apply_url = apply_url.removesuffix('/apply')
        # --- MODIFICATION END ---


        # --- Build the Typst block for one job ---
        current_job_lines = []

        # 1. Header with Title and linked Job ID
        # Typst: = Heading, #link("url")[text]
        # URLs within Typst strings might need escaping for backslashes and quotes
        safe_apply_url = apply_url.replace("\\", "\\\\").replace('"', '\\"')
        new_tag = " (New)" if new == 1 else ""
        header = (
            f"= {escape_typst(title)} (\#link(\"{safe_apply_url}\")[Job ID: {escape_typst(job_id)}]{new_tag})"
        )
        current_job_lines.append(header)

        # 2. Metadata Table
        metadata_map = {
            "Candidate Match Score": "candidate_match_score",
            "New Job since 20251210 (1=Yes,0=No)": "new",
            "Slide-tag relevance": "slide_tag_relevance",
            "Worker type": "worker_type",
            "Sub category": "sub_category",
            "Job profile": "job_profile",
            "Supervisory organization": "supervisory_organization",
            "Target hire date": "target_hire_date",
            "Openings": "openings",
            "Grade profile": "grade_profile",
            "Recruiting start date": "recruiting_start_date",
            "Job level": "job_level",
            "Grade": "grade",
            "Job family": "job_family",
            "Is evergreen": "is_evergreen",
        }

        table_cell_contents = []
        for display_name, column_name in metadata_map.items():
            value = row.get(column_name)
            if pd.notna(value) and value != "":
                # Ensure integer values don't have decimals (e.g., openings, scores)
                if isinstance(value, float) and value.is_integer():
                    value = int(value)
                # Typst table cells are comma-separated. The first column is bold.
                table_cell_contents.append(f"  [* {escape_typst(display_name)} *], [{escape_typst(value)}],")

        if table_cell_contents:
            # Table properties are set globally using #set table(...), so we just define the content here.
            current_job_lines.append("#table(\n" + "\n".join(table_cell_contents) + "\n)")


        # 3. Summary Section
        js = row.get("job_summary")

        def try_parse_list(value):
            if isinstance(value, list): return value
            if not isinstance(value, str): return None
            text = value.strip()
            if not text: return None
            try:
                parsed = json.loads(text)
                if isinstance(parsed, list): return parsed
            except Exception: pass
            try:
                parsed = ast.literal_eval(text)
                if isinstance(parsed, list): return parsed
            except Exception: pass
            return None

        summary_list = try_parse_list(js)

        if summary_list:
            # Typst: == Subheading
            current_job_lines.append("\n== Summary")
            # Typst lists: - [Item content]
            for item in summary_list:
                if item is not None and str(item).strip():
                    current_job_lines.append(f"- [{escape_typst(item)}]")
        elif isinstance(js, str) and js.strip():
            # Fallback for summaries that are just plain text
            current_job_lines.append("\n== Summary")
            current_job_lines.append(f"[{escape_typst(js)}]") # Wrap in [] for a content block

        typst_job_blocks.append("\n".join(current_job_lines))

    if not typst_job_blocks:
        print(f"No jobs found with candidate_match_score >= {min_candidate_score}")
        return

    # --- Assemble the final document ---
    # Join each job block with a pagebreak command
    full_typst_content = (
            TYPST_PREAMBLE
            + ("\n\n#pagebreak()\n\n".join(typst_job_blocks))
            + "\n" + TYPST_POSTAMBLE
    )

    # Print to stdout
    print(full_typst_content)

    # Optionally save to file
    if out_path:
        try:
            # Change file extension from .tex to .typ
            typst_out_path = out_path.replace(".tex", ".typ")
            with open(typst_out_path, "w", encoding="utf-8") as f:
                f.write(full_typst_content)
            print(f"\n--- SUCCESS ---\nTypst written to {typst_out_path}")
        except Exception as e:
            print(f"Failed to write Typst to {typst_out_path}: {e}")


# ==============================================================================
# --- MAIN EXECUTION ---
# ==============================================================================

# read in the dataframe from previous step
try:
    # Make sure this CSV file exists and is in the correct path
    df_jobs = pd.read_csv("df_with_candidate_match.csv")
    df_jobs_old = pd.read_csv("20251210/me/df_with_candidate_match.csv")
except FileNotFoundError:
    print("Error: 'df_with_candidate_match.csv' not found.")
    print("Creating a dummy DataFrame for demonstration purposes.")
    # Create a sample DataFrame if the file doesn't exist
    dummy_data = {
        'job_id': ['202507-119341', '202508-121705', '202507-118937'],
        'new' : [1, 1, 0],
        'title': ['Bioanalytical Assay Developer', 'Stability Manager', 'Leiter Daten Governance'],
        'apply_url': ['http://example.com/apply/202507-119341/apply', 'http://example.com/apply/202508-121705', 'http://example.com/apply/202507-118937'],
        'worker_type': ['Angestellt', 'Angestellt', 'Angestellt'],
        'sub_category': ['Research', 'Quality', 'IT'],
        'job_profile': ['Scientist', 'Manager', 'Manager'],
        'supervisory_organization': ['Bio-Analytics (John Doe)', 'Quality Control (Jane Smith)', 'Data Office (Max Mustermann)'],
        'recruiting_start_date': ['2025-07-01', '2025-08-15', '2025-07-20'],
        'job_level': ['Individual Contributor', 'Manager', 'Manager'],
        'job_family': ['Research', 'Quality', 'IT'],
        'is_evergreen': [0, 0, 1],
        'slide_tag_relevance': [4.0, 1.0, 4.0],
        'candidate_match_score': [5.0, 4.0, 1.0],
        'target_hire_date': [None, '2025-11-01', None],
        'openings': [1.0, 1.0, 1.0],
        'grade_profile': ['CH_All_PL5 Research & Development', 'CH_All_PL6 Quality', 'CH_All_PL7 IT'],
        'grade': ['PL5', 'PL6', 'PL7'],
        'job_summary': [
            '["Develop regulatory-compliant bioanalytical assays.", "Expertise in ligand binding assays for PK, PD, and immunogenicity.", "Contribute to and author regulatory documents."]',
            '["Manage and ensure GMP-compliant stability programs.", "Author and review stability sections of regulatory submissions."]',
            '["Define and execute PT-wide data governance strategy.", "Lead data governance initiatives across the organization."]'
        ]
    }
    df_jobs = pd.DataFrame(dummy_data)
except Exception as e:
    print(f"Failed to read 'df_with_candidate_match.csv': {e}")
    df_jobs = None

# create a new column 'new' marking jobs that are new (1) or old (0)
# for each row in df_jobs, check if job_id is present in df_jobs_old
if df_jobs is not None and 'df_jobs_old' in globals() and df_jobs_old is not None:
    df_jobs['new'] = df_jobs['job_id'].apply(
        lambda x: 1 if x not in df_jobs_old['job_id'].values else 0)

if "df_jobs" in globals() and df_jobs is not None:
    try:
        # This will create the .typ file you can compile with typst
        # The .tex extension in out_path will be replaced with .typ
        jobs_to_typst(df_jobs, min_candidate_score=3, out_path="high_score_jobs.tex")
    except Exception as e:
        print(f"Failed to produce Typst document: {e}")
