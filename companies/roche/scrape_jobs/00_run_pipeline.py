#!/usr/bin/env python3
"""
Master orchestration script for the Roche Job Scraper & Analysis Pipeline.

This script automates the complete workflow:
1. Creates a new dated folder for this run
2. Executes scripts 01-05 to collect and process general job information
3. For each candidate profile, executes 05b-07c to generate candidate-specific reports

Usage:
    # Run pipeline for a single candidate (outputs to <date>/<candidate>/...)
    uv run 00_run_pipeline.py <candidate_profile_path>

    # With custom AI model and parameters
    uv run 00_run_pipeline.py <candidate_profile_path> \\
        --model gemini-2.5-flash-lite \\
        --max-len 150000 \\
        --max-word-limit 4000

    # Provide a specific previous date for new job comparison
    uv run 00_run_pipeline.py <candidate_profile_path> \\
        --previous-date 20260201

    # Run multiple candidates sequentially (reuses 01-05 results)
    uv run 00_run_pipeline.py <candidate1_profile_path> --candidate-name Alice
    uv run 00_run_pipeline.py <candidate2_profile_path> --candidate-name Bob

Arguments:
    candidate_profile_path: Path to the candidate profile text file

Options:
    --candidate-name TEXT           Name/identifier for the candidate (default: extracted from filename)
    --skip-general               Skip scripts 01-05 (useful when running multiple candidates)
    --skip-candidate             Skip scripts 05b-07c (useful for general job collection only)
    --model TEXT                 Model name for AI (default: gemini-3-flash-preview for 05b, gemini-flash-latest for 05)
    --max-len INT                Max characters per API request for 05 (default: 200000)
    --max-word-limit INT         Max words per API request for 05b (default: 5100)
    --previous-date TEXT         Date folder of previous run (YYYYMMDD) for new job highlighting
    --no-typst                   Skip Typst generation (07b and 07c)
    --dry-run                    Print commands without executing
"""

import argparse
import sys
import os
import subprocess
from datetime import datetime
from pathlib import Path


def run_command(cmd, dry_run=False, description=""):
    """Execute a shell command and handle errors."""
    if description:
        print(f"\n{'='*80}")
        print(f"  {description}")
        print('='*80)

    print(f"Command: {' '.join(cmd)}")

    if dry_run:
        print("[DRY RUN] Command not executed")
        return True

    try:
        result = subprocess.run(cmd, check=True, capture_output=False)
        return result.returncode == 0
    except subprocess.CalledProcessError as e:
        print(f"Error: Command failed with exit code {e.returncode}")
        return False
    except Exception as e:
        print(f"Error: {e}")
        return False


def get_candidate_name(profile_path, override_name=None):
    """Extract candidate name from profile path or use override."""
    if override_name:
        return override_name

    # Extract filename without extension
    name = Path(profile_path).stem
    # Remove common prefixes like 'candidate_' or 'profile_'
    for prefix in ['candidate_', 'profile_', 'cv_', 'resume_']:
        if name.lower().startswith(prefix):
            name = name[len(prefix):]
    return name or "candidate"


def main():
    parser = argparse.ArgumentParser(
        description="Master orchestration script for Roche Job Scraper Pipeline",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )

    parser.add_argument(
        "candidate_profile_path",
        help="Path to the candidate profile text file"
    )
    parser.add_argument(
        "--candidate-name",
        default=None,
        help="Name/identifier for the candidate (default: extracted from filename)"
    )
    parser.add_argument(
        "--skip-general",
        action="store_true",
        help="Skip scripts 01-05 (general job collection)"
    )
    parser.add_argument(
        "--skip-candidate",
        action="store_true",
        help="Skip scripts 05b-07c (candidate-specific processing)"
    )
    parser.add_argument(
        "--model",
        default=None,
        help="Model name for AI (default: gemini-3-flash-preview for 05b, gemini-flash-latest for 05)"
    )
    parser.add_argument(
        "--model-05",
        default=None,
        help="Model name for script 05 (default: gemini-flash-latest)"
    )
    parser.add_argument(
        "--model-05b",
        default=None,
        help="Model name for script 05b (default: gemini-3-flash-preview)"
    )
    parser.add_argument(
        "--max-len",
        type=int,
        default=200000,
        help="Max characters per API request for 05 (default: 200000)"
    )
    parser.add_argument(
        "--max-word-limit",
        type=int,
        default=5100,
        help="Max words per API request for 05b (default: 5100)"
    )
    parser.add_argument(
        "--previous-date",
        default=None,
        help="Date folder of previous run (YYYYMMDD) for new job highlighting"
    )
    parser.add_argument(
        "--no-typst",
        action="store_true",
        help="Skip Typst generation (07b and 07c)"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print commands without executing"
    )

    args = parser.parse_args()

    # Validate candidate profile exists
    if not os.path.exists(args.candidate_profile_path):
        print(f"Error: Candidate profile not found: {args.candidate_profile_path}")
        sys.exit(1)

    # Get absolute path for candidate profile
    candidate_profile_path = os.path.abspath(args.candidate_profile_path)
    candidate_name = get_candidate_name(candidate_profile_path, args.candidate_name)

    # Create dated folder
    date_str = datetime.now().strftime("%Y%m%d")
    date_folder = Path(date_str)
    date_folder.mkdir(exist_ok=True)

    # Create candidate subfolder
    candidate_folder = date_folder / candidate_name
    candidate_folder.mkdir(exist_ok=True)

    print(f"\n{'='*80}")
    print(f"  Roche Job Scraper & Analysis Pipeline")
    print(f"{'='*80}")
    print(f"Date: {date_str}")
    print(f"Candidate: {candidate_name}")
    print(f"Output folder: {candidate_folder}")
    print(f"Candidate profile: {candidate_profile_path}")
    print(f"Dry run: {args.dry_run}")
    print(f"{'='*80}\n")

    # Set model defaults if not specified
    model_05 = args.model_05 or args.model or "gemini-flash-latest"
    model_05b = args.model_05b or args.model or "gemini-3-flash-preview"

    success = True

    # =========================================================================
    # PHASE 1: General job collection (scripts 01-05)
    # =========================================================================
    if not args.skip_general:
        print(f"\n{'*'*80}")
        print("PHASE 1: General Job Collection (Scripts 01-05)")
        print(f"{'*'*80}\n")

        # Script 01: Scrape job links
        if not run_command(
            ["uv", "run", "01_main.py"],
            dry_run=args.dry_run,
            description="Step 01: Scraping job links from Roche careers website"
        ):
            success = False

        # Script 02: Download job pages
        if not run_command(
            ["uv", "run", "02_fetchlinks.py"],
            dry_run=args.dry_run,
            description="Step 02: Downloading job pages"
        ):
            success = False

        # Script 03: Extract JSON from HTML
        if not run_command(
            ["bash", "-c", "for f in jobs_html/*.html; do uv run 03_extract_job_info.py \"$f\"; done"],
            dry_run=args.dry_run,
            description="Step 03: Extracting JSON from HTML files"
        ):
            success = False

        # Script 04: Populate database
        if not run_command(
            ["bash", "-c", "uv run 04_json_to_sqlite.py jobs_html/*.json"],
            dry_run=args.dry_run,
            description="Step 04: Populating SQLite database"
        ):
            success = False

        # Script 05: Filter and annotate with AI
        cmd_05 = ["uv", "run", "05_db_filter.py"]
        if model_05 != "gemini-flash-latest":
            cmd_05.extend(["--model", model_05])
        if args.max_len != 200000:
            cmd_05.extend(["--max-len", str(args.max_len)])

        if not run_command(
            cmd_05,
            dry_run=args.dry_run,
            description="Step 05: Filtering jobs and adding AI annotations"
        ):
            success = False

        if not success:
            print("\nError: General job collection failed. Exiting.")
            sys.exit(1)

    # =========================================================================
    # PHASE 2: Candidate-specific processing (scripts 05b-07c)
    # =========================================================================
    if not args.skip_candidate:
        print(f"\n{'*'*80}")
        print(f"PHASE 2: Candidate-Specific Processing (Scripts 05b-07c)")
        print(f"Candidate: {candidate_name}")
        print(f"{'*'*80}\n")

        # Change to candidate folder for output
        original_cwd = os.getcwd()
        os.chdir(candidate_folder)

        try:
            # Script 05b: Match jobs to candidate
            cmd_05b = ["uv", "run", "../../05b_match_candidate.py", candidate_profile_path]
            if model_05b != "gemini-3-flash-preview":
                cmd_05b.extend(["--model", model_05b])
            if args.max_word_limit != 5100:
                cmd_05b.extend(["--max-word-limit", str(args.max_word_limit)])

            if not run_command(
                cmd_05b,
                dry_run=args.dry_run,
                description=f"Step 05b: Matching jobs to {candidate_name}"
            ):
                success = False

            if success and not args.no_typst:
                # Script 07b: Generate Typst document
                cmd_07b = ["uv", "run", "../../07b_jobs_to_typst.py"]
                if args.previous_date:
                    cmd_07b.extend(["--previous-date", f"../../{args.previous_date}"])

                if not run_command(
                    cmd_07b,
                    dry_run=args.dry_run,
                    description=f"Step 07b: Generating Typst document for {candidate_name}"
                ):
                    success = False

                # Script 07c: Generate all jobs Typst document
                cmd_07c = ["uv", "run", "../../07c_all_jobs_to_typst.py"]
                if args.previous_date:
                    cmd_07c.extend(["--previous-date", f"../../{args.previous_date}"])

                if not run_command(
                    cmd_07c,
                    dry_run=args.dry_run,
                    description=f"Step 07c: Generating all jobs Typst document for {candidate_name}"
                ):
                    success = False

        finally:
            os.chdir(original_cwd)

    # =========================================================================
    # Summary
    # =========================================================================
    print(f"\n{'='*80}")
    if success:
        print("  ✓ Pipeline completed successfully!")
    else:
        print("  ✗ Pipeline completed with errors. Please review the output above.")
    print(f"{'='*80}\n")

    if not args.dry_run:
        print(f"Output folder: {os.path.abspath(candidate_folder)}")
        print(f"\nTo view results:")
        if not args.no_typst:
            print(f"  typst compile {candidate_folder}/high_score_jobs.typ")
            print(f"  typst compile {candidate_folder}/high_score_jobs_all.typ")

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()

