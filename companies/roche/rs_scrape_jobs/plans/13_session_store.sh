#!/bin/bash

# This script prepares a comprehensive session store implementation plan for the Rust job scraping project
# addressing the critical session persistence problem and requesting detailed architectural guidance.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Du bist ein Rust-Architekt und musst uns helfen.
Eine duemmere KI als du muss deine Anweisungen implementieren also
musst du sehr detailliert und präzise sein. Vielleicht auch code beispiele angeben.

ich kann ein profil eingeben und matches ermitteln. wenn ich mich auslogge und wieder einlogge sehe
ich die matches nicht mehr. die sollten erhalten bleiben

Die AI kann über deepwiki MCP auf Dokumentation von tower-sessions und libsql und anderen dependencies zugreifen, aber die Implementierung muss zu unserer aktuellen Architektur passen.


EOF

declare -a FILES=(
    "$ROOT_DIR/plans/12_visualisierung_review.md"  # Status des Projekts
    "$ROOT_DIR/Cargo.toml"                        # Dependencies und Versionen
    "$ROOT_DIR/src/11_web_server.rs"              # Session Store Initialisierung
    "$ROOT_DIR/src/12_auth.rs"                    # OAuth und Session Handling
    "$ROOT_DIR/src/13_web_ui.rs"                  # Session Nutzung in UI
    "$ROOT_DIR/src/01b_db_repo.rs"                # LibSQL Connection und DB-Logik
    "$ROOT_DIR/src/01c_db_traits.rs"              # Database Traits
    "$ROOT_DIR/src/15_app_state.rs"               # AppState und DB Provider
    "$ROOT_DIR/templates/base.html"               # Navigation und User State
    "$ROOT_DIR/templates/dashboard.html"          # Session-abhängige UI
    "$ROOT_DIR/templates/profile.html"            # Profile Management
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

echo "Session Store Implementierungsplan wurde in die Zwischenablage kopiert."
echo "Enthält Analyse des Persistenz-Problems, Custom SessionStore Implementierung und Integration."
echo "Fokus: tower-sessions 0.15.0 + libsql Store mit production-ready features."
