#!/bin/bash

# This script sends a comprehensive architectural request to Rust architect LLM
# for implementing AI summarization button feature in admin console.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Du bist ein Rust-Architekt und musst uns helfen.
Eine dümmere KI als du muss deine Anweisungen implementieren, also
musst du sehr detailliert und präzise sein. Gib konkrete Code-Beispiele.

der bisherig code erlaubt nicht, die zusammenfassungen von job descriptions per AI
ueber die admin console zu generieren. fuege die entsprechende funktionalitaet ein
idealerweise, soll dies auch geschehen, wenn scraping durchgefuehrt wird.


Benutze bei deiner umsetzung gute software architektur und programming patterns um den
code modular und maintainable zu halten.

Eine duemmere KI als du muss deine Anweisungen implementieren also
musst du sehr detailliert und präzise sein. Vielleicht auch code beispiele angeben.
du musst den implementierungsplan in kleinere schritte aufteilen, die in angebrachter reihenfolge ausgefuehrt werden.
gib an wann programm neu compiliert werden sollte und wann etwaige tests ausgefuehrt werden sollen um 
die bisherige implementierung zu verifizieren

Die AI fuer die implementierung kann über deepwiki MCP auf Dokumentation von dependencies zugreifen.
Dafür muss das Planungsdokument aber die Github Organisation und den Namen des entsprechenden Projekts enthalten. 

Fuer unser project ist es plops/slide-tag

EOF

declare -a FILES=(
    "$ROOT_DIR/src/15_app_state.rs"
    "$ROOT_DIR/src/17_admin.rs"
    "$ROOT_DIR/templates/admin.html"
    "$ROOT_DIR/src/01c_db_traits.rs"
    "$ROOT_DIR/src/01b_db_repo.rs"
    "$ROOT_DIR/src/07b_ai_gemini.rs"
    "$ROOT_DIR/src/00_models.rs"
    "$ROOT_DIR/src/11_web_server.rs"
    "$ROOT_DIR/src/07d_ai_rate_limiter.rs"
    "$ROOT_DIR/src/07e_ai_batch_builder.rs"
    "$ROOT_DIR/src/07_ai_core.rs"
    "$ROOT_DIR/src/bin/main.rs"
    "$ROOT_DIR/tests/web_integration_e2e.rs"
    "$ROOT_DIR/Cargo.toml"
    "$ROOT_DIR/README.md"
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

echo "AI Summarization Button Architect Request wurde in die Zwischenablage kopiert."
