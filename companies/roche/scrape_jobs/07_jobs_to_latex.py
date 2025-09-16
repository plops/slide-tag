import pandas as pd
import ast
import json
import re

# --- Helper function to escape special LaTeX characters ---
def escape_latex(text: str) -> str:
    """
    Escapes special LaTeX characters in a given string.
    """
    if not isinstance(text, str):
        text = str(text)

    # Order of replacement is important
    replacements = {
        '\\': r'\textbackslash{}',
        '&': r'\&',
        '%': r'\%',
        '$': r'\$',
        '#': r'\#',
        '_': r'\_',
        '{': r'\{',
        '}': r'\}',
        '~': r'\textasciitilde{}',
        '^': r'\textasciicircum{}',
    }

    # Use a regex to perform all replacements in one pass
    # This avoids issues where one replacement creates a sequence that another would match
    regex = re.compile('|'.join(re.escape(key) for key in replacements.keys()))
    return regex.sub(lambda match: replacements[match.group(0)], text)

# --- LaTeX Preamble Definition ---
LATEX_PREAMBLE = r"""
\documentclass[11pt, a4paper]{article}

% --- PDFLATEX COMPATIBLE PREAMBLE ---
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{sourcesanspro}
\renewcommand*\familydefault{\sfdefault}

% --- OTHER PACKAGES ---
\usepackage[margin=1in]{geometry}
\usepackage{tabularray}
\UseTblrLibrary{booktabs}
\usepackage{hyperref}
\hypersetup{
    colorlinks=true,
    linkcolor=blue,
    filecolor=magenta,      
    urlcolor=blue,
    pdftitle={Job Descriptions},
    pdfauthor={Automated Script},
}
\usepackage{enumitem}

\begin{document}
"""

# --- LaTeX Postamble (End of Document) ---
LATEX_POSTAMBLE = r"\end{document}"


def jobs_to_latex(
        df: pd.DataFrame, min_relevance: int = 4, out_path: str | None = None
):
    """
    Generate a LaTeX document for jobs with slide_tag_relevance >= min_relevance.
    Writes the output to out_path if provided.
    """
    if df is None:
        print("jobs_to_latex: no dataframe provided")
        return

    latex_job_blocks = []

    # --- Re-use relevance and sorting logic from your original script ---
    def relevance_of(x):
        try:
            return int(x)
        except Exception:
            try:
                return int(float(x))
            except Exception:
                return None

    dfc = df.copy()
    dfc["_slide_tag_relevance_num"] = pd.to_numeric(
        dfc.get("slide_tag_relevance"), errors="coerce"
    )
    df_sorted = dfc.sort_values(
        by="_slide_tag_relevance_num", ascending=False
    )
    # --- End of re-used logic ---

    for _, row in df_sorted.iterrows():
        rel = relevance_of(row.get("slide_tag_relevance"))
        if rel is None or rel < min_relevance:
            continue

        job_id = row.get("job_id", "")
        title = row.get("title", "")
        apply_url = row.get("apply_url", "")

        # --- Build the LaTeX block for one job ---
        current_job_lines = []

        # 1. Header with Title and linked Job ID
        header = (
            f"\\section*{{{escape_latex(title)} \\quad "
            f"(\\href{{{apply_url}}}{{Job ID: {escape_latex(job_id)}}})}}"
        )
        current_job_lines.append(header)

        # 2. Metadata Table
        # Define which columns to include and their display names
        metadata_map = {
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
            "Slide-tag relevance": "slide_tag_relevance",
        }

        table_rows = []
        for display_name, column_name in metadata_map.items():
            value = row.get(column_name)
            if pd.notna(value) and value != "":
                # Ensure integer values don't have decimals (e.g., openings)
                if isinstance(value, float) and value.is_integer():
                    value = int(value)
                table_rows.append(f"{escape_latex(display_name)} & {escape_latex(value)} \\\\")

        if table_rows:
            current_job_lines.append("\\begin{tblr}{ colspec = {Q[l, font=\\bfseries] X[l]}, hlines, }")
            current_job_lines.extend(table_rows)
            current_job_lines.append("\\end{tblr}")

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
            current_job_lines.append("\n\\subsection*{Summary}")
            current_job_lines.append("\\begin{itemize}[leftmargin=*]")
            for item in summary_list:
                if item is not None and str(item).strip():
                    current_job_lines.append(f"    \\item {escape_latex(item)}")
            current_job_lines.append("\\end{itemize}")
        elif isinstance(js, str) and js.strip():
            # Fallback for summaries that are just plain text
            current_job_lines.append("\n\\subsection*{Summary}")
            current_job_lines.append(escape_latex(js))

        latex_job_blocks.append("\n".join(current_job_lines))

    if not latex_job_blocks:
        print(f"No jobs found with slide_tag_relevance >= {min_relevance}")
        return

    # --- Assemble the final document ---
    # Join each job block with a \newpage command
    full_latex_content = (
            LATEX_PREAMBLE
            + ("\n\n\\newpage\n\n".join(latex_job_blocks))
            + "\n" + LATEX_POSTAMBLE
    )

    # Print to stdout
    print(full_latex_content)

    # Optionally save to file
    if out_path:
        try:
            with open(out_path, "w", encoding="utf-8") as f:
                f.write(full_latex_content)
            print(f"\n--- SUCCESS ---\nLaTeX written to {out_path}")
        except Exception as e:
            print(f"Failed to write LaTeX to {out_path}: {e}")


# ==============================================================================
# --- MAIN EXECUTION ---
# ==============================================================================

# read in the dataframe from previous step
try:
    # Make sure this CSV file exists and is in the correct path
    df_slide = pd.read_csv("df_with_ai_annotations.csv")
except FileNotFoundError:
    print("Error: 'df_with_ai_annotations.csv' not found.")
    print("Creating a dummy DataFrame for demonstration purposes.")
    # Create a sample DataFrame if the file doesn't exist
    dummy_data = {
        'job_id': ['202402-104044', '202507-117910'],
        'title': ['Entwicklungsingenieur', 'Group Leader'],
        'apply_url': ['https://roche.wd3.myworkdayjobs.com/roche-ext/job/Rotkreuz/Development-Engineer--contract-80-100--_202402-104044-1/apply', 'https://roche.wd3.myworkdayjobs.com/roche-ext/job/Basel/Group-Leader---Computational-Medicine---Imaging-Data-Insights--pRED_202507-117910/apply'],
        'worker_type': ['Angestellt', 'Angestellt'],
        'sub_category': ['Production Engineering', 'Research'],
        'job_profile': ['Development Engineer', 'Scientific Management'],
        'supervisory_organization': ['DSRMGJ NAP/qPCR & Sequencing (Vahid Akbarzadeh) (32410074)', 'GTAE Computational CoE (Jörg Degen) (50682980)'],
        'recruiting_start_date': ['2024-02-19', '2025-07-15'],
        'job_level': ['Individual Contributor', 'Manager with direct reports'],
        'job_family': ['Production Engineering', 'Research'],
        'is_evergreen': [1, 0],
        'slide_tag_relevance': [5.0, 5.0],
        'target_hire_date': [None, '2025-10-01'],
        'openings': [None, 1.0],
        'grade_profile': [None, 'CH_All_PL8 Research & Development'],
        'grade': [None, 'PL8'],
        'job_summary': [
            '["Work as a Development Engineer in System Development (R&D) to optimize workflows and integrate hardware, software, consumables, and reagents for diagnostic systems.", "Elaborate on robotics concepts (Cobots) for laboratory automation towards a \'dark lab\' vision.", "Knowledge of professional software development with languages like UR-Polyscope, Python, C#/NET, or LabView is essential."]',
            '["Lead a local team of imaging experts in Basel and Penzberg.", "Deep knowledge in biomedical image analytics, AI/ML, and imaging biomarker research.", "Proven ability to lead multidisciplinary teams, handle ambiguity, and achieve timely results with multiple stakeholders."]'
        ]
    }
    df_slide = pd.DataFrame(dummy_data)
except Exception as e:
    print(f"Failed to read 'df_with_ai_annotations.csv': {e}")
    df_slide = None


if "df_slide" in globals() and df_slide is not None:
    try:
        # This will create the .tex file you can compile with pdflatex
        jobs_to_latex(df_slide, min_relevance=4, out_path="high_relevance_jobs.tex")
    except Exception as e:
        print(f"Failed to produce LaTeX document: {e}")