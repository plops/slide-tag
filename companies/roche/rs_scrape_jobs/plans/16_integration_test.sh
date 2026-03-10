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

Ich möchte einen Integrationstest für die Webseite bauen, das heißt, die Knöpfe sollen gedrückt
werden und zum Beispiel das Einloggen mittels GitHub Oauth soll erprobt werden. 

Außerdem möchte ich checken, dass die Logik funktioniert, wenn ein Nutzer sein Profil eingibt und im Prinzip den ganzen Workflow durcharbeiten. 
Die wesentlichen Workflows sind dann:
ohne eingeloggt zu sein durch die Jobprofile browsen. 
 einloggen
 einen Eintrag machen mit den ins Profil. 
außerdem Matches suchen anhand des profils.
von der Match Übersichtsliste auf die Details gehen. 
Als Administrator das Scraping der Webseite veranlassen. 

Welche Programmiersprache würdest du davor verwenden? Würdest du da auch Rust für die Tests nehmen? Oder wäre Python besser? Und welche Art, den Browser zu kontrollieren? In Frage kommen Selenium, Helium, Playwright und vielleicht auch die Library, die wir jetzt schon benutzen, Chromium Oxide, für Rust? 

Die AI kann über deepwiki MCP auf Dokumentation von dependencies zugreifen.
Dafür muss das Planungsdokument aber die Github Organisation und den Namen des Projekts enthalten. 


EOF

declare -a FILES=(
    "$ROOT_DIR/plans/03_tasks.md"
    "$ROOT_DIR/Cargo.toml"
    "$ROOT_DIR/src/"*.rs
    "$ROOT_DIR/src/bin/"*.rs
    "$ROOT_DIR/templates/"*.html
    "$ROOT_DIR/tests/"*.rs
    "$HOME/stage/slide-tag/.github/workflows/"*
    "$ROOT_DIR/docs/release_process.md"
    "$ROOT_DIR/deployment/templates/"*
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
