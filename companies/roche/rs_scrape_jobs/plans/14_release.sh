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


i added a release build in Cargo.toml that i run on my hetzner server. it is hosted on jobs.rocketrecap.ocm

currently the login using the github outh doesn't work for my webistes users (github gives a 404). i ihave the secrets on the server (for github and gemini) in a .env file.

i have configuration from the server in config_release/ which reveals more than i want about folder structure on my serevrer (and where the .env is)

deine aufgabe ist die gesamte code base zu reviewen.
ist der zustand der software ausreichend um auf dem server zu deployen?
ausserdem moechte ich die wesentliche punkte der server configuration im  git repo
haben ohne die wirklich sensitive informationen preiszugeben.

weiterhin brauche ich einen github workflow und eine erklaerung welche secrets 
ich auf github  wie einrichten muss, damit der release prozess dort von einer action erzeugt wird 
ich habe einen github workflow von einem anderen projekt eingefuegt. dieser muss angepasst werden

update auch docs/release_process.md

Die AI kann über deepwiki MCP auf Dokumentation von tower-sessions und libsql und anderen dependencies zugreifen, aber die Implementierung muss zu unserer aktuellen Architektur passen.


EOF

declare -a FILES=(
    "$ROOT_DIR/plans/03_tasks.md"
    "$ROOT_DIR/Cargo.toml"
    "$ROOT_DIR/src/"*.rs
    "$ROOT_DIR/src/bin/"*.rs
    "$ROOT_DIR/templates/"*.html
    "$ROOT_DIR/tests/"*.rs
    "$ROOT_DIR/config_release/"*
    "$HOME/stage/slide-tag/.github/workflows/"*
    "$ROOT_DIR/docs/release_process.md"
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

echo "Release Implementierungsplan wurde in die Zwischenablage kopiert."
