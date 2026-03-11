#!/bin/bash

# This script sends a comprehensive architectural request to the Rust architect LLM
# for optimizing the AI workflow in the job scraping application.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

Du bist ein Rust-Architekt und musst uns helfen.
Eine dümmere KI als du muss deine Anweisungen implementieren, also
musst du sehr detailliert und präzise sein. Gib konkrete Code-Beispiele.

## AUFGABENÜBERSICHT

### TEIL 1: ANALYSE (20%)
1. **Current AI Workflow Analysis**: 
   - Gib eine detaillierte Übersicht aller KI-Anfragen, die aktuell an Gemini gesendet werden
   - Dokumentiere den genauen Workflow von Job-Scraping → AI-Processing → Ergebnisse und den Workflow fuer Candidate Matching mittels AI
   - Identifiziere aktuelle Engpässe und Optimierungspotenziale

### TEIL 2: ARCHITEKTUR-ENTWURF (80%)

#### 2.1 Job-Zusammenfassungs-System
**Anforderung**: Erstelle Bullet-Point-Zusammenfassungen für Job-Beschreibungen
- **Input**: Vollständige Job-Beschreibung (typisch 2 Seiten) + ATS-Daten (job_level, job_family, grade_profile)
- **Output**: Strukturierte Bullet-Points im `job_summary` Feld
- **Batching**: Kombiniere so viele jobs pro API-Anfrage dass 25000 worte (im  request) nicht ueberschritten werden zur Optimierung der RPD-Limits

#### 2.2 Fortschritts-Logging-System
**Anforderung**: Detailliertes Logging während der AI-Verarbeitung
- **Format**: "Processing job descriptions 18..23 out of 180"
- **Granularität**: Logge jeden Batch und jede einzelne Job-Verarbeitung
- **Metriken**: Verarbeitungszeit, Token-Verbrauch, API-Antwortzeiten

#### 2.3 Dual-Control Rate Limiting
**Anforderung**: Unabhängige Steuerung von Wortanzahl und TPM
- **Neue Konfiguration**: `words_per_request` (Standard: 1000 Wörter)
- **TPM-Behaltung**: Behalte 250.000 TPM-Limit bei
- **RPM-Behaltung**: Behalte 15 RPM-Limit bei
- **Logik**: Erlaube mehrere kurze Anfragen pro Minute innerhalb aller Limits

#### 2.4 Admin-Only Job-Level Display
**Anforderung**: Job-Level nur für Admin-Benutzer (plops) anzeigen
- **Template-Änderung**: `templates/jobs.html` - bedingte Anzeige von `job_level`
- **Auth-Check**: Nutze existierende `is_admin()` Funktion aus `src/17_admin.rs`
- **UI-Integration**: Füge Job-Level in Job-Liste und Detail-Ansicht ein

#### 2.5 Optimiertes Candidate-Matching
**Anforderung**: Nutze Job-Zusammenfassungen statt voller Beschreibungen
- **Input**: Kandidatenprofil + Job-Zusammenfassungen (nicht volle Beschreibungen)
- **Batching**: Mehrere Matches pro API-Anfrage (aber jeder request soll unter 25000 worten bleiben)
- **Performance**: Ziel: 3x schnellere Verarbeitung bei gleicher Qualität aber gleichzeitig sparsam mit der Anzahl der requests

## ERWARTETE LIEFERUNGEN

1. **Architektur-Dokumentation**: Detailliertes Design der neuen Systeme
2. **Code-Beispiele**: Implementierung der Kern-Komponenten
3. **Konfigurations-Schema**: YAML/JSON für neue Parameter
4. **Migration-Plan**: Wie von current zu new architecture
5. **Performance-Benchmark**: Vorher/Nachher Vergleich

## FRAGEN ZUR KLÄRUNG
- Welche ATS-Felder außer `job_level` sind relevant für die Zusammenfassung?
- Sollen die Zusammenfassungen mehrsprachig sein (DE/EN)?
- Wie soll mit fehlgeschlagenen API-Aufrufen umgegangen werden (Retry-Logik)?


Eine duemmere KI als du muss deine Anweisungen implementieren also
musst du sehr detailliert und präzise sein. Vielleicht auch code beispiele angeben.
du musst den implementierungsplan in kleinere schritte aufteilen, die in angebrachter reihenfolge ausgefuehrt werden.
gib an wann programm neu compiliert werden sollte und wann etwaige tests ausgefuehrt werden sollen um 
die bisherige implementierung zu verifizieren

Die AI fuer die implementierung kann über deepwiki MCP auf Dokumentation von dependencies zugreifen.
Dafür muss das Planungsdokument aber die Github Organisation und den Namen des Projekts enthalten. 



EOF

declare -a FILES=(
    "$ROOT_DIR/src/07_ai_core.rs"
    "$ROOT_DIR/src/07b_ai_gemini.rs"
    "$ROOT_DIR/src/07d_ai_rate_limiter.rs"
    "$ROOT_DIR/src/07e_ai_batch_builder.rs"
    "$ROOT_DIR/src/00_models.rs"
    "$ROOT_DIR/src/13_web_ui.rs"
    "$ROOT_DIR/src/17_admin.rs"
    "$ROOT_DIR/templates/jobs.html"
    "$ROOT_DIR/src/bin/stage6_ai_test.rs"
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

echo "AI Architect Request wurde in die Zwischenablage kopiert."
