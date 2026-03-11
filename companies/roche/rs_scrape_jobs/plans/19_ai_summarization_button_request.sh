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

## AUFGABENÜBERSICHT

### TEIL 1: ANALYSE (20%)
1. **Current Admin System Analysis**: 
   - Gib eine detaillierte Übersicht der aktuellen Admin-Konsole in `src/17_admin.rs`
   - Analysiere die existierende Scrape-Status-Verwaltung und Routing-Struktur
   - Identifiziere wie die `is_admin()` Funktion und Session-Management funktioniert

2. **AI Infrastructure Analysis**:
   - Dokumentiere die aktuelle AI-Provider-Architektur in `src/07b_ai_gemini.rs`
   - Analysiere die `annotate_jobs` Methode und Batch-Verarbeitungslogik
   - Untersuche Rate Limiting und Token-Management in `src/07d_ai_rate_limiter.rs`

3. **Database Integration Analysis**:
   - Analysiere die `DatabaseProvider` trait in `src/01c_db_traits.rs`
   - Untersuche `get_unannotated_jobs` und `update_job_ai` Methoden
   - Dokumentiere die Job-Modelle in `src/00_models.rs` speziell `JobAnnotation`

### TEIL 2: ARCHITEKTUR-ENTWURF (80%)

#### 2.1 AI Status Management System
**Anforderung**: Erstelle Status-Tracking für AI-Operationen
- **AppState Erweiterung**: Füge `ai_status: Arc<RwLock<AiStatus>>` Feld hinzu
- **AiStatus Enum**: Implementiere `Idle`, `Running {start_time, processed_jobs, total_jobs, current_batch}`, `Error(String)`, `Success {completion_time, processed_count}`
- **Thread Safety**: Nutze `Arc<RwLock<>>` für sicheren Status-Zugriff
- **Initialisierung**: Füge ai_status zu allen AppState-Konstruktoren hinzu

#### 2.2 Admin Console UI Integration
**Anforderung**: Erstelle AI-Kontrollsektion in Admin-Dashboard
- **Template Update**: `templates/admin.html` mit AI-Status-Display
- **Button Logic**: Deaktiviere Button während AI-Verarbeitung
- **Progress Display**: Zeige "X/Y jobs" Fortschritt in Echtzeit
- **Status Colors**: Orange für laufend, Grün für idle/success, Rot für Fehler
- **Unannotated Count**: Zeige Anzahl der wartenden Jobs

#### 2.3 Background Processing Architecture
**Anforderung**: Implementiere nicht-blockierende AI-Verarbeitung
- **Handler Function**: `post_trigger_ai_summarization` mit Admin-Auth
- **Background Task**: Nutze `tokio::spawn` für asynchrone Verarbeitung
- **Status Updates**: Aktualisiere ai_status während Verarbeitung
- **Error Handling**: Graceful Fehlerbehandlung mit Status-Updates

#### 2.4 AI Processing Pipeline
**Anforderung**: Integriere existierende AI-Infrastruktur
- **Job Retrieval**: Nutze `get_unannotated_jobs(limit)` aus DatabaseProvider
- **AI Processing**: Rufe `ai_provider.annotate_jobs(jobs)` auf
- **Batch Management**: AI-Provider handle interne Batching und Rate Limits
- **Database Updates**: Speichere Ergebnisse mit `update_job_ai(identifier, summary)`
- **Progress Tracking**: Update Status alle 10 Jobs oder bei Abschluss

#### 2.5 Route Integration
**Anforderung**: Füge AI-Trigger-Route zu Admin-Routing hinzu
- **Route Definition**: `POST /admin/ai/trigger` mit `post_trigger_ai_summarization`
- **Admin Routes**: Erweitere `admin_routes()` Funktion um neue Route
- **Web Server**: Verifiziere Integration in `src/11_web_server.rs`
- **Authentication**: Stelle sicher dass nur Admins Zugriff haben

#### 2.6 DeepWiki MCP Integration
**Anforderung**: Nutze DeepWiki MCP für Dependency-Verständnis
- **GitHub Access**: Organisation `plops` Repository `slide-tag`
- **Askama Docs**: Template-Engine und Formular-Handling
- **Axum Docs**: Routing, State-Management und Async-Handler
- **Tokio Docs**: Async Runtime und Task-Spawning
- **SQLx Docs**: Database-Operations und Trait-Implementierung

## ERWARTETE LIEFERUNGEN

1. **Architektur-Dokumentation**: Detailliertes Design aller Komponenten
2. **Code-Implementierung**: Vollständige Rust-Implementierung mit Imports
3. **Template-Integration**: Askama-Template mit UI-Logik
4. **Route-Konfiguration**: Axum-Router mit Admin-Schutz
5. **Status Management**: Thread-sichere AI-Status-Verwaltung
6. **Test-Strategie**: Unit- und Integration-Tests
7. **Deployment-Plan**: Wie neue Features integriert werden

## TECHNISCHE ANFORDERUNGEN

### Code-Qualität
- Nutze `async/await` für alle Database-Operationen
- Implementiere proper Error-Handling mit `anyhow::Result`
- Folge existierenden Code-Patterns und Naming-Konventionen
- Nutze `tracing` für Logging und Debug-Information

### Performance
- Background-Verarbeitung darf UI nicht blockieren
- Rate Limiting muss beachtet werden (25.000 Wörter/request)
- Status-Updates müssen effizient sein (minimale Locks)
- Memory-Usage bei großen Job-Mengen optimieren

### Sicherheit
- Admin-Authentifizierung zwingend erforderlich
- Keine direkten Datenbank-Zugriff ohne Validierung
- Session-Management muss sicher sein
- Input-Validierung für alle Parameter

## FRAGEN ZUR KLÄRUNG
- Wie sollen Concurrent AI-Requests gehandhabt werden (Queue/Mutex)?
- Soll die AI-Verarbeitung abgebrochen werden können?
- Wie sollen Timeouts und Retry-Logik implementiert werden?
- Welche zusätzlichen Status-Informationen sind nützlich?

Eine duemmere KI als du muss deine Anweisungen implementieren also
musst du sehr detailliert und präzise sein. Vielleicht auch code beispiele angeben.
du musst den implementierungsplan in kleinere schritte aufteilen, die in angebrachter reihenfolge ausgefuehrt werden.
gib an wann programm neu compiliert werden sollte und wann etwaige tests ausgefuehrt werden sollen um 
die bisherige implementierung zu verifizieren

Die AI fuer die implementierung kann über deepwiki MCP auf Dokumentation von dependencies zugreifen.
Dafür muss das Planungsdokument aber die Github Organisation und den Namen des Projekts enthalten. 

GitHub Organisation: plops
Repository Name: slide-tag
Projekt Pfad: /home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs


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
