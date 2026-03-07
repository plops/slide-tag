#!/bin/bash
# rs_scrape_jobs/plans/01_requirements.sh

# This script collects the Python codebase and reference materials for porting to Rust.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche"
SCRAPE_JOBS_DIR="${ROOT_DIR}/scrape_jobs"

{
declare -a FILES=(
    "${SCRAPE_JOBS_DIR}/pyproject.toml"
    "${SCRAPE_JOBS_DIR}/README.md"
    "${SCRAPE_JOBS_DIR}/INDEX.md"
    "${SCRAPE_JOBS_DIR}/EXAMPLE_WORKFLOWS.md"
    "${SCRAPE_JOBS_DIR}/00_run_pipeline.py"
    "${SCRAPE_JOBS_DIR}/01_main.py"
    "${SCRAPE_JOBS_DIR}/02_fetchlinks.py"
    "${SCRAPE_JOBS_DIR}/03_extract_job_info.py"
    "${SCRAPE_JOBS_DIR}/04_json_to_sqlite.py"
    "${SCRAPE_JOBS_DIR}/05_db_filter.py"
    "${SCRAPE_JOBS_DIR}/05b_match_candidate.py"
    "${SCRAPE_JOBS_DIR}/07b_jobs_to_typst.py"
    "../docs/deps/libsql.md"
    "../docs/deps/rig.md"
    "../docs/deps/chromiumoxide.md"
    "../docs/deps/genai.md"
    "../docs/deps/llm-chain.md"
    "../docs/deps/clap.md"
    "../docs/deps/serde.md"
    "../docs/deps/tera.md"
    "../docs/deps/askama.md"
    "../docs/deps/typst-as-library.md"
    "../docs/deps/typst-core.md"
    "01b_rust_port_plan.md"
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

cat << 'EOF'

erzeuge einen umfangreichen implemntierungsplan fuer den rust port.
ueberlege welche architektur und programming pattern am besten geeignet sind um das program umzusetzen.
arbeite einen stufenweisen plan aus, wo die einzelmodule individuell compiliert und getestet werden koennen.

z.b. zuerst datenbank anlegen und lesen. 
danach die erste webseite benutzen (nur schauen dass es geht)
dann die webseite benutzen und eine suche durchfuehren.
erst dann html links erfassen und herunterladen.
und danach erst das json extrahieren und in datei speichern.
wir werden uns die daten anschauen und
und dann erst die informationen in die datenbank schreiben.

da rust dependencies schnell anwachsen koennen, muessen wir
bei jedem einzelmodul darauf achten die abhaengigkeiten zu minimieren

(z.b. mit carge features)

der plan soll fuer jeden der entwicklungsschritte die zu erzeugenden dateien und 
verzeichnishierarchien enthalten. wenn eine datei zu gross wird, dann sollen sie
nach thema gesplittet werden. jede einzeldatei soll eine nummer am anfang haben (01_... 02_...)
so dass man sich schnell einarbeiten kann um den wichtigsten code zu finden.

EOF
} | xclip -selection clipboard

echo "Codebase and updated Agentic Rust porting prompt (local-first) have been copied to the clipboard."
