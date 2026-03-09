#!/bin/bash

# This script prepares a comprehensive session store implementation plan for the Rust job scraping project
# addressing the critical session persistence problem and requesting detailed architectural guidance.

ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs"

{

cat << 'EOF'

--- PROMPT ---

**SESSION STORE IMPLEMENTIERUNG - KRITISCHES PROBLEM LÖSEN**

Das Rust Job-Scraping-Projekt hat eine grundlegende Session-Persistenz-Problem, die dringend gelöst werden muss. Wir benötigen eine detaillierte Analyse und Implementierungsanleitung für einen persistenten Session Store.

**PROBLEM-BESCHREIBUNG:**

**Was wir versucht haben:**
1. **tower-sessions-libsql-store** versucht zu integrieren
2. **Versionskonflikt entdeckt:** tower-sessions-libsql-store 0.4.0 ist nur mit tower-sessions 0.10.0-0.11.1 kompatibel
3. **Wir verwenden:** tower-sessions 0.15.0 → **INKOMPATIBEL!**
4. **Zurückgefallen auf:** MemoryStore (nicht persistent)

**Unser aktuelles Problem:**
- **Sessions gehen bei Server-Neustart verloren** (MemoryStore)
- **Profile und Matches verschwinden** beim Aus/Einloggen
- **User Experience ist schlecht:** Nutzer müssen Daten neu eingeben
- **Production nicht ready:** Keine persistente Session-Verwaltung

**TECHNISCHE SITUATION:**

**Current Stack:**
- Rust + Axum Web Framework
- tower-sessions 0.15.0 für Session Management
- libsql (Turso) als Hauptdatenbank
- OAuth2 GitHub Login
- Askama Templates für HTML Rendering

**Current Session Setup:**
```rust
// src/11_web_server.rs - aktuelle MemoryStore Implementierung
let session_store = MemoryStore::default();
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnInactivity(TsDuration::hours(1)));
```

**WARUM LIBSQL STORE DIE RICHTIGE WAHL IST:**

**Vorteile vs Cookie Store:**
- ✅ **Persistenz über Server-Neustarts** (genau unser Problem!)
- ✅ **Unbegrenzte Session-Daten** (Profile, Matches, etc.)
- ✅ **Sicher:** Nur Session ID im Cookie, Daten server-seitig
- ✅ **Voll Kontrolle:** Sessions können server-seitig invalidiert werden
- ✅ **Nutzt bestehende Infrastruktur:** Wir haben bereits libsql

**IMPLEMENTIERUNGS-ANFORDERUNGEN:**

**1. Custom SessionStore Trait Implementierung:**
Wir müssen den `SessionStore` trait für libsql implementieren:

```rust
#[async_trait]
pub trait SessionStore: Debug + Send + Sync + 'static {
    async fn create(&self, record: &mut Record) -> Result<(), Error>;
    async fn save(&self, record: &Record) -> Result<(), Error>;
    async fn load(&self, session_id: &Id) -> Result<Option<Record>, Error>;
    async fn delete(&self, session_id: &Id) -> Result<(), Error>;
}
```

**2. Production-Ready Features:**
- **ID Collision Handling:** Bei Session ID Konflikten neue ID generieren
- **Expired Session Filtering:** Abgelaufene Sessions automatisch ausfiltern
- **Background Cleanup:** Periodische Säuberung alter Sessions
- **Thread Safety:** Send + Sync für并发 Nutzung
- **Error Handling:** LibSQL Fehler zu SessionStore::Error mappen

**3. Datenbank-Schema für Sessions:**
```sql
-- Sessions Tabelle für libsql
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL,  -- JSON serialisierte Session-Daten
    expiry_date INTEGER NOT NULL,  -- Unix timestamp
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Index für Performance
CREATE INDEX IF NOT EXISTS idx_sessions_expiry ON sessions(expiry_date);
```

**4. Integration mit bestehender App:**
- Session Store in `create_app()` integrieren
- Migration für Sessions-Tabelle hinzufügen
- Background Task für Cleanup starten
- Error Handling und Logging implementieren

**ANALYSE-ANFORDERUNGEN:**

1. **Aktuelle Session-Implementierung analysieren:**
   - Wie funktioniert der aktuelle MemoryStore?
   - Welche Session-Daten werden gespeichert?
   - Wie ist die Integration mit Axum?

2. **LibSQL Integration analysieren:**
   - Wie ist unsere aktuelle libsql Connection aufgebaut?
   - Welche Connection Pool verwenden wir?
   - Wie können wir Sessions in die bestehende DB integrieren?

3. **Architektur-Review:**
   - Wo im Code wird der SessionStore initialisiert?
   - Wie werden Sessions in den Handlern verwendet?
   - Welche Abhängigkeiten müssen angepasst werden?

**GEWÜNSCHTE IMPLEMENTIERUNGSPHASEN:**

**Phase 1: Custom SessionStore Implementieren (Must Have)**
- `SessionStore` trait für libsql implementieren
- Sessions-Tabelle erstellen und migrieren
- Grundlegende CRUD-Operationen für Sessions
- Integration mit tower-sessions 0.15.0

**Phase 2: Production Features (Should Have)**
- ID Collision Handling implementieren
- Expired Session Filtering
- Background Cleanup Task
- Proper Error Handling und Logging

**Phase 3: Performance & Monitoring (Nice to Have)**
- Connection Pooling optimieren
- Session Statistics und Monitoring
- Caching Layer (optional)
- Backup/Recovery Strategie

**TECHNISCHE ANFORDERUNGEN:**

Bitte analysiere den gesamten Quellcode und erstelle einen detaillierten Implementierungsplan, der:

1. **SessionStore Trait Implementierung:** Volle, production-ready Implementierung
2. **Datenbank-Integration:** Sessions in unsere libsql Datenbank integrieren
3. **Migration-Skript:** Schema für Sessions-Tabelle
4. **Background Cleanup:** Automatische Säuberung alter Sessions
5. **Error Handling:** Robuste Fehlerbehandlung und Logging

**IMPLEMENTIERUNGS-ANLEITUNG:**

Da die Implementierung durch ein LLM erfolgt, musst du:

- **Schritt-für-Schritt Anleitung:** Jede Methode detailliert erklären
- **Konkrete Code-Beispiele:** Vollständige Implementierung mit Typen
- **Dependencies auflisten:** Genau welche crates benötigt werden
- **Integration Points:** Wo im Code die Änderungen nötig sind
- **Testing Strategie:** Wie die Implementierung getestet wird
- **Migration Plan:** Wie von MemoryStore zu libsql Store gewechselt wird

Die AI kann über deepwiki MCP auf Dokumentation von tower-sessions und libsql zugreifen, aber die Implementierung muss zu unserer aktuellen Architektur passen.

**ERWARTETES ERGEBNIS:**

Ein vollständiger Implementierungsplan mit:
1. Detaillierter Analyse der aktuellen Session-Architektur
2. Komplette Custom SessionStore Implementierung für libsql
3. Schritt-für-Schritt Anleitung zur Integration
4. Production-ready Features und Best Practices
5. Migration-Strategie von MemoryStore zu libsql Store

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
