#!/bin/bash

# This script prepares a comprehensive review and implementation planning for the next phase
# of the Rust job scraping project, addressing specific shortcomings and requesting detailed
# architectural review.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Das Rust Job-Scraping-Projekt hat eine grundlegende Visualisierung implementiert, aber es gibt wichtige Mängel und fehlende Features. Wir benötigen eine detaillierte Überprüfung und einen neuen Implementierungsplan.

**AKTUELLE PROBLEME UND FEHLENDE FEATURES:**

1. **Keine Job-Detailseiten**: 
   - Es gibt keine Möglichkeit, Detailseiten für einzelne Jobs anzuzeigen
   - Fehlende Informationen: Paygrade, Hiring Manager, Dates, Standort, etc.
   - Nutzer können nicht tief in Job-Details eintauchen

2. **Fehlende Navigation**:
   - Kein "Jobs" Link in der Top-Bar für normale Nutzer
   - Jobs-Link existiert nur auf Home/Profile/Dashboard Seiten
   - Inkonsistente Navigationserfahrung

3. **Persistenz-Probleme**:
   - Profile und Summaries verschwinden bei jedem Webserver-Neustart
   - Daten werden nicht dauerhaft gespeichert
   - Nutzer müssen Daten neu eingeben

4. **Startseiten-Problem**:
   - Startseite leitet sofort zu GitHub weiter
   - Keine eigentliche Landing Page oder Dashboard
   - Schlechte User Experience für neue Nutzer

5. **Allgemeine Code-Qualität**:
   - Viele Änderungen wurden implementiert
   - Benötigt detaillierte Überprüfung der gesamten Architektur
   - Mögliche technische Schulden und Inkonsistenzen

**ANALYSE-ANFORDERUNGEN:**

1. **Datenstruktur-Analyse**:
   - Überprüfe die aktuelle Datenbankstruktur für Jobs und Matches
   - Analysiere, warum Profile/Summaries nicht persistieren
   - Bewerte die Datenmodellierung für Job-Details

2. **Architektur-Review**:
   - Überprüfe das gesamte Routing und Handler-System
   - Analysiere die Template-Struktur und Navigation
   - Bewerte die Session-Management und Persistenz

3. **User Experience Review**:
   - Analysiere den kompletten User Flow von Login zu Job-Ansicht
   - Identifiziere Navigationsschwachstellen
   - Bewerte die Responsive Design Implementation

4. **Code-Qualitäts-Audit**:
   - Überprüfe Error Handling und Logging
   - Analysiere Performance und Datenbankabfragen
   - Bewerte Security und OAuth Integration

**GEWÜNSCHTE IMPLEMENTIERUNGSPHASEN:**

**Phase 1: Kritische Fehler beheben (Must Have)**
- Persistenz für Profile und Summaries implementieren
- Job-Detailseiten mit vollständigen Informationen
- Konsistente Navigation mit Top-Bar Jobs-Link
- Eigene Startseite statt GitHub-Weiterleitung

**Phase 2: User Experience verbessern (Should Have)**
- Bessere Job-Detailansicht mit allen Metadaten
- Verbesserte Filter und Sortierung
- Mobile-Optimierung für Detailseiten
- Besseres Error Handling

**Phase 3: Advanced Features (Nice to Have)**
- Job-Vergleichsfunktionen
- Erweiterte Statistiken und Charts
- Export-Funktionen
- Admin-Dashboard

**TECHNISCHE ANFORDERUNGEN:**

Bitte analysiere den gesamten Quellcode und erstelle einen detaillierten Implementierungsplan, der:

1. **Datenbank-Persistenz**: Löse das Problem, dass Profile/Summaries nicht gespeichert werden
2. **Navigation-System**: Implementiere konsistente Navigation mit Top-Bar Links
3. **Detailseiten**: Erstelle vollständige Job-Detailseiten mit allen Informationen
4. **Startseite**: Implementiere eine proper Landing Page
5. **Code-Qualität**: Identifiziere und behebe technische Schulden

**IMPLEMENTIERUNGS-ANLEITUNG:**

Da die Implementierung durch ein LLM erfolgt, das weniger clever ist und weniger Context behalten kann, musst du:

- **Architektur klar definieren**: Wie die Komponenten zusammenarbeiten
- **Datenstrukturen spezifizieren**: Genaue Modelle und Beziehungen
- **Programming Patterns nennen**: Best Practices für Rust Webentwicklung
- **Arbeitsschritte aufteilen**: Kleine, verständliche Einzelschritte
- **Beispielquellcode geben**: Konkrete Implementierungsbeispiele
- **Dependencies dokumentieren**: Nutzung von crates wie axum, sqlx, etc.

Die AI kann über deepwiki MCP auf Dokumentation der Dependencies zugreifen, aber die Gesamtstruktur muss passen.

**ERWARTETES ERGEBNIS:**

Ein priorisierter Implementationsplan mit:
1. Detaillierter Analyse der aktuellen Probleme
2. Konkreten Lösungsansätzen mit Code-Beispielen
3. Schritt-für-Schritt Anleitung zur Implementierung
4. Architektur-Verbesserungen für langfristige Wartbarkeit

EOF

declare -a FILES=(
    "$ROOT_DIR/plans/03_tasks.md"
    "$ROOT_DIR/plans/11_visualisierung.md"
    "$ROOT_DIR/Cargo.toml"
    "$ROOT_DIR/src/"*.rs
    "$ROOT_DIR/src/bin/"*.rs
    "$ROOT_DIR/templates/"*.html
    "$ROOT_DIR/tests/"*.rs
    "$ROOT_DIR/migrations/"*.sql
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

echo "Visualisierungs-Review und neuer Implementierungsplan wurde in die Zwischenablage kopiert."
echo "Enthält Analyse von Persistenz-Problemen, Navigation, Detailseiten und Startseite."
