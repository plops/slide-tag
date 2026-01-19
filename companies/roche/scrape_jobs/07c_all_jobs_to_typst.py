import pandas as pd
import ast
import json
import re

# --- Helper function to escape special Typst characters ---
def escape_typst(text: str) -> str:
    """
    Escapes special Typst markup characters in a given string.
    This ensures that the text is treated as literal content within Typst content blocks.
    """
    if not isinstance(text, str):
        text = str(text)

    # Typst special characters to escape within content blocks `[...]`
    # Escape the backslash itself first to prevent it from escaping other characters
    # Order matters: \ needs to be handled before [ and ]
    replacements = {
        '\\': r"\\",  # Escape the escape character
        '[': r"\[",   # Literal open bracket
        ']': r"\]",   # Literal close bracket
        '*': r"\*",   # Literal asterisk (for bold)
        '_': r"\_",   # Literal underscore (for italic)
        '`': r"\`",   # Literal backtick (for raw text)
        '$': r"\$",   # Literal dollar sign (for math)
        '#': r"\#",   # Literal hash (for function calls)
        '~': r"\~",   # Literal tilde (for non-breaking space)
        '^': r"\^",   # Literal caret (for superscript)
        '{': r"\{",   # Literal open curly brace
        '}': r"\}",   # Literal close curly brace
    }

    # Use a regex to perform all replacements in one pass
    # This avoids issues where one replacement creates a sequence that another would match
    regex = re.compile('|'.join(re.escape(key) for key in replacements.keys()))
    escaped_text = regex.sub(lambda match: replacements[match.group(0)], text)
    return escaped_text

# --- Typst Global Settings Definition (equivalent to a preamble) ---
TYPST_PREAMBLE = r"""
#set text(font: "Fira Sans", lang: "en")
#set page(
  margin: (top: 1in, bottom: 1in, left: 1in, right: 1in),
)
#set heading(numbering: none) // Unnumbered headings for jobs and summaries
//#set link(stroke: blue) // Styling for hyperlinks
//#set list(marker: auto) // Default bullet for lists
#set table(
  stroke: (x: 0.5pt, y: 0.5pt), // All lines 0.5pt
  //fill: (
  //  even: luma(250), // Optional: light grey background for even rows
  //  odd: white,
  //),
  inset: 5pt, // Padding inside cells
  align: horizon, // Horizontal alignment of cell content
)
"""

# --- Typst Postamble (no equivalent, document simply ends) ---
TYPST_POSTAMBLE = r""


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

    # --- Sort dataframe by recruiting_start_date (earliest first) -- format: YYYY-MM-DD ---
    dfc = df.copy()
    dfc["candidate_match_score"] = pd.to_numeric(
        dfc.get("candidate_match_score"), errors="coerce"
    )
    dfc["recruiting_start_date"] = pd.to_datetime(
        dfc.get("recruiting_start_date"), errors="coerce"
    )
    df_sorted = dfc.sort_values(
        by="recruiting_start_date", ascending=True, na_position="last"
    )
    # --- End of sorting logic ---

    for _, row in df_sorted.iterrows():
        score = row.get("candidate_match_score")


        job_id = row.get("job_id", "")
        is_new = row.get("new", 0) == 1
        title = row.get("title", "")
        apply_url = row.get("apply_url", "")

        # --- MODIFICATION START ---
        # If the URL ends with /apply, remove it
        apply_url = apply_url.removesuffix('/apply')
        # --- MODIFICATION END ---

        # --- Build the Typst block for one job ---
        current_job_lines = []

        # 1. Header with Title and linked Job ID
        title_escaped = escape_typst(title)
        job_id_escaped = escape_typst(job_id)
        
        job_id_link_content = f"#link(\"{apply_url}\")[Job ID: {job_id_escaped}]"
        
        new_text = f" {escape_typst('(New)')}" if is_new else ""
        
        header = (
            f"#heading(level: 1, outlined: false, [{title_escaped} #h(1em) ({job_id_link_content}{new_text})])"
        )
        current_job_lines.append(header)

        # 2. Metadata Table
        metadata_map = {
            "Candidate Match Score": "candidate_match_score",
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

        table_cells = []
        for display_name, column_name in metadata_map.items():
            value = row.get(column_name)
            if pd.notna(value) and value != "":
                # Ensure integer values don't have decimals (e.g., openings, scores)
                if isinstance(value, float) and value.is_integer():
                    value = int(value)
                table_cells.append(f"[* {escape_typst(display_name)} *]") # Bold the display name
                table_cells.append(f"[{escape_typst(value)}]")

        if table_cells:
            # Typst table structure: #table(columns: ..., cell1, cell2, cell3, cell4, ...)
            current_job_lines.append("\n#table(")
            current_job_lines.append("  columns: (auto, 1fr),") # Key column auto-width, value column fills rest
            current_job_lines.append("  table.header([*Attribute*], [*Value*]),") # Table header
            current_job_lines.extend([f"  {cell}," for cell in table_cells]) # Add comma after each cell content
            current_job_lines.append(")")

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
            current_job_lines.append("\n#heading(level: 2, outlined: false, [Summary])")
            current_job_lines.append("#list(")
            for item in summary_list:
                if item is not None and str(item).strip():
                    current_job_lines.append(f"  [{escape_typst(item)}],")
            current_job_lines.append(")")
        elif isinstance(js, str) and js.strip():
            # Fallback for summaries that are just plain text
            current_job_lines.append("\n#heading(level: 2, outlined: false, [Summary])")
            current_job_lines.append(f"[{escape_typst(js)}]")

        typst_job_blocks.append("\n".join(current_job_lines))

    if not typst_job_blocks:
        print(f"No jobs found with candidate_match_score >= {min_candidate_score}")
        return

    # --- Assemble the final document ---
    # Join each job block with a pagebreak
    full_typst_content = (
            TYPST_PREAMBLE
            + "\n\n#pagebreak()\n\n".join(typst_job_blocks)
            + TYPST_POSTAMBLE # This is an empty string, but keeps the structure
    )

    # Print to stdout
    print(full_typst_content)

    # Optionally save to file
    if out_path:
        # Change file extension to .typ
        typst_out_path = out_path.rsplit('.', 1)[0] + '.typ' if '.' in out_path else out_path + '.typ'
        try:
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
    df_jobs_old = pd.read_csv("20251210/df_with_candidate_match.csv")
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
        jobs_to_typst(df_jobs, min_candidate_score=3, out_path="high_score_jobs.typ")
    except Exception as e:
        print(f"Failed to produce Typst document: {e}")
