#!/bin/bash

# This script collects the Rust codebase design documents in 
# order to help changing the design before finalizing the implementation.


ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Schaue die Cargo.toml an. Optimiere die Abhaengigkeiten der Packete auf das Minimum.
Weiterhin zeige moeglichkeiten auf compilierzeit zu optimieren und memoryverbrauch
waehrend der Compilierung zu reduzieren. Das Programm muss nicht schnell sein.

Mein editor soll nicht abstuerzen!

EOF

declare -a FILES=(
    "$ROOT_DIR/plans/03_tasks.md"
    "$ROOT_DIR/Cargo.toml"
    "$ROOT_DIR/src/"*.rs
    "$ROOT_DIR/src/bin/"*.rs
)

for i in "${FILES[@]}"; do
    if [ -f "$i" ]; then
        SIZE_KB=$(du -k "$i" | cut -f1)
        echo "LOG: Processing $i (${SIZE_KB} KB)" >&2
        echo "// start of $i"
        cat "$i"
        echo "// end of $i"
    else
        echo "ERROR: File not found: $i" >&2
        exit 1
    fi
done


} | xclip -selection clipboard

echo "Codebase and updated Plan updating prompt have been copied to the clipboard."
