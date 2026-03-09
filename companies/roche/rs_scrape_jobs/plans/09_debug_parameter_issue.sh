#!/bin/bash

# This script collects all source files needed to debug the parameter count issue
# with libsql/rusqlite INSERT statements in the job_history table.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

I'm having a parameter count mismatch issue with a Rust database INSERT statement using libsql/rusqlite. 

**Problem**: When trying to insert a job record into the job_history table, I get the error:
"SQLite failure: `35 values for 33 columns`"

**Context**: 
- I'm using libsql (which is built on SQLite) with the rusqlite params! macro
- The job_history table has 34 columns total (including auto-increment 'id')
- My INSERT statement excludes the 'id' column, so it should insert into 33 columns
- But somehow 35 values are being provided instead of 33

**What I need you to analyze**:
1. Check the database schema in 01_db_setup.rs - verify the exact column count
2. Examine the INSERT statement in 01b_db_repo.rs - count the columns and parameters
3. Look at the Job model in 00_models.rs - see if there are any field mismatches
4. Check if there are any issues with the params! macro usage or None value handling

**Key files to focus on**:
- src/01_db_setup.rs (database schema)
- src/01b_db_repo.rs (INSERT statement with the error)
- src/00_models.rs (Job struct definition)
- src/01c_db_traits.rs (DatabaseProvider trait)

**Expected outcome**: 
- Identify why there are 35 values instead of 33
- Suggest the correct fix for the INSERT statement
- Explain any rusqlite/libsql parameter handling nuances I might be missing

Please provide a detailed analysis of the parameter mismatch and suggest the exact code changes needed.

EOF

declare -a FILES=(
    "$ROOT_DIR/src/01_db_setup.rs"
    "$ROOT_DIR/src/01b_db_repo.rs"
    "$ROOT_DIR/src/00_models.rs"
    "$ROOT_DIR/src/01c_db_traits.rs"
    "$ROOT_DIR/src/bin/test_insert.rs"
    "$ROOT_DIR/src/bin/debug_insert.rs"
    "$ROOT_DIR/Cargo.toml"
)

for i in "${FILES[@]}"; do
    if [ -f "$i" ]; then
        SIZE_KB=$(du -k "$i" | cut -f1)
        echo "LOG: Processing $i (${SIZE_KB} KB)" >&2
        echo "// start of $i"
        cat "$i"
        echo "// end of $i"
        echo ""
        echo ""
    else
        echo "WARNING: File not found: $i" >&2
    fi
done

} | xclip -selection clipboard

echo "Parameter issue debugging files and prompt have been copied to the clipboard."
echo "Files included:"
printf "  - %s\n" "${FILES[@]}"
