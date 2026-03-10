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

Mache einen Review vom aktuellen code.
Angenommen ein admin hat `rs-scrape serve` als systemd service laufen,
wie kann sie einen scrape triggern? Ist die richtige vorgehensweise, den webserver
anzuhalten und dann den scrape zu triggern? falls das so ist, ist das eine komische
art. besser waere man kann dies fuer den laufenden service triggern.

mache einen entsprechenden implementierungsplan.

am besten waere es, wenn ich meinen github account (github user name plops) als admin nutzen
koennte und mittels einer admin schnittstelle diese sache (und  vielleicht auch andere parameter)
steuern und ueberwachen koennte.

Die AI kann über deepwiki MCP auf Dokumentation von tower-sessions und libsql und anderen dependencies zugreifen, aber die Implementierung muss zu unserer aktuellen Architektur passen.


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
