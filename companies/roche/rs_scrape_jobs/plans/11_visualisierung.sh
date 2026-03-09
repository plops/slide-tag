#!/bin/bash

# This script prepares the visualization implementation planning by collecting all relevant
# source code, documentation and planning files. The focus is on implementing a comprehensive
# visualization system for job matches and all entries.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Die meisten Features des Rust Job-Scraping-Projekts sind implementiert. Jetzt geht es um die Implementierung der Visualisierung für Job-Matches und alle Einträge.

Analysiere den gesamten Quellcode und erstelle einen detaillierten Implementierungsplan für:

1. **Match-Visualisierung (Priorität 1)**: 
   - Darstellung der gefundenen Job-Matches für Nutzer (must have)
   - Filter- und Sortiermöglichkeiten nach Relevanz, Datum (nice to have)
   - Detailansicht für einzelne Matches mit allen relevanten Informationen (must have)

2. **Gesamtübersicht aller Einträge**: 
   - Visualisierung aller gescrapten Job-Einträge im System (must have)
   - Administrative Ansicht für System-Überwachung (nice to have)
   - Statistische Auswertungen und Trends (nice to have)

3. **Datenvisualisierungskomponenten**:
   - Interaktive Tabellen mit Pagination (nice have)
   - Charts und Graphen für Job-Markt-Statistiken (nice to have)
   - Such- und Filterfunktionen mit Echtzeit-Updates (nice have)

4. **User Interface Integration**:
   - Nahtlose Integration in bestehendes Template-System
   - Responsive Design für mobile und Desktop-Ansicht (nice to have)
   - Barrierefreie Gestaltung (Accessibility)

5. **Performance-Optimierung für Visualisierung**:
   - Effiziente Datenabfragen aus der Datenbank
   - Caching-Strategien für häufig abgerufene Daten
   - Lazy Loading für große Datenmengen

6. **Export-Funktionalitäten**:
   - CSV/Excel Export für Matches und Statistiken (not important yet)
   - PDF-Berichte für administrative Zwecke (not important yet)
   - Druckoptimierte Ansichten (not important yet)

Versetze dich in die Lage der Nutzer und des Administrators des Servers.

**Für Nutzer**: Können Nutzer ihre Matches einfach und übersichtlich anzeigen? Verstehen sie sofort welche Jobs am besten passen? Können sie Matches vergleichen und priorisieren?

**Für Administratoren**: Haben Administratoren einen klaren Überblick über das System? Können sie scraping-Erfolge und Datenqualität schnell bewerten? (not important, the existing logging is enough)

Berücksichtige dabei:
- Die bestehende Architektur mit den implementierten Features
- Die Datenbankstruktur mit Job-Einträgen und Matches
- Die bestehenden Templates und CSS-Frameworks
- Die Web-Server Struktur und Routing
- Die OAuth-Integration und Nutzer-spezifische Daten

Erstelle einen priorisierten Implementationsplan mit konkreten Schritten für die Match-Visualisierung als erstes Ziel.


Gehe davon aus, dass die implementierung durch ein LLM erfolgt, dass weniger clever ist als du und auch weniger im context behalten kann. Darum musst du im implementierungsplan deutlich die architektur und massgeblich datenstrukturen definieren, programming patterns erwaehnen und die arbeitsaufgaben definieren. wenn moeglich gib auch beispielquellcode an. die ai kann mittels deepwiki mcp auf die dokumentation der dependencies zugreifen und damit detailprobleme loesen. wichtig ist dass die gesamtstruktur passt.

EOF

declare -a FILES=(
    "$ROOT_DIR/plans/03_tasks.md"
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

echo "Visualisierungs-Implementierungsplanung mit Quellcode und Dokumentation wurde in die Zwischenablage kopiert."
