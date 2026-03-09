#!/bin/bash

# This script prepares the final integration planning by collecting all relevant
# source code, documentation and planning files. The focus is on combining and
# integrating all implemented components into a cohesive system.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Die meisten Features des Rust Job-Scraping-Projekts sind implementiert. Jetzt geht es um die finale Integration und Kombination aller Komponenten.

Analysiere den gesamten Quellcode und erstelle einen detaillierten Plan für:

1. **Komponenten-Integration**: Wie alle Module (Datenbank, Web-Server, Scraper, Scheduler, etc.) zusammenspielen
2. **Datenfluss-Optimierung**: Effiziente Datenverarbeitung zwischen den Komponenten
3. **Konfigurationsmanagement**: Zentrale Verwaltung aller Einstellungen
4. **Error Handling**: Konsistente Fehlerbehandlung über alle Module hinweg
5. **Testing-Strategie**: Integrationstests für das Gesamtsystem
6. **Deployment-Vorbereitung**: Schritte für den produktiven Einsatz
7. **Performance-Optimierung**: Identifikation von Engpässen und Optimierungspotenzial
8. **Monitoring & Logging**: Zentrales Logging und Monitoring für Betriebssicherheit

Versetze dich in die Lage der Nutzer und des Administrators des Servers.
Koennen Nutzer sich ein und ausloggen, ihre Daten eingeben und fuer sie passende Jobs sehen?

Kann der Administrator die Anwendung starten, das naechtliche scraping automatisch oder auch manuell starten?
Ueberlege dir welche Use cases fuer diesen Webserver noch noetig sind und bewerte die bisherige umsetzung der loesung.

Berücksichtige dabei:
- Die bestehende Architektur mit den implementierten Features
- Die Datenbankstruktur und Repository-Pattern
- Die Web-Oberfläche mit Templates
- Die Scraper-Logik und Scheduler-Funktionalität
- Die OAuth-Integration und Benutzerverwaltung

Erstelle einen priorisierten Aktionsplan mit konkreten Schritten für die finale Integration.

EOF

declare -a FILES=(
    "$ROOT_DIR/plans/03_tasks.md"
    "$ROOT_DIR/plans/01b_rust_port_plan.md"
    "$ROOT_DIR/STAGE11_IMPLEMENTATION_SUMMARY.md"
    "$ROOT_DIR/STAGE12_IMPLEMENTATION_SUMMARY.md"
    "$ROOT_DIR/Cargo.toml"
    "$ROOT_DIR/src/"*.rs
    "$ROOT_DIR/src/bin/"*.rs
    "$ROOT_DIR/templates/"*.html
    "$ROOT_DIR/tests/"*.rs
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

echo "Endplanungsvorbereitung mit Quellcode und Dokumentation wurde in die Zwischenablage kopiert."
