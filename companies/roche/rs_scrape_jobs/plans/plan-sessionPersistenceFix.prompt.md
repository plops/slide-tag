# Plan: Session-Persistenz im OAuth-Flow reparieren

## Problem

Das Session-Cookie geht bei Redirects verloren, weil der `SessionManagerLayer` ohne korrekte Cookie-Konfiguration initialisiert wird. Nach GitHub-OAuth-Login existiert die Session nicht mehr, daher wird die Startseite ohne Authentifizierung angezeigt.

## Root Cause

In `src/11_web_server.rs` Zeile 9-10:
```rust
let session_store = MemoryStore::default();
let session_layer = SessionManagerLayer::new(session_store);
```

Der `SessionManagerLayer` wird mit Default-Einstellungen erstellt, die folgende Probleme verursachen:

1. **Keine Cookie-Persistenz**: Session-Cookie wird nicht mit `SameSite::Lax` konfiguriert
2. **Keine Expiry-Policy**: Sessions enden möglicherweise sofort nach Request
3. **Fehlende Secure-Flags**: Cookie wird nicht für HTTPS konfiguriert
4. **Keine Domain-Konfiguration**: Cookie-Scope ist nicht definiert

## Implementierungsschritte

### 1. Cookie-Konfiguration in `11_web_server.rs`

**Datei:** `src/11_web_server.rs`

**Änderungen:**
- Importiere `tower_sessions::cookie::Cookie` und `tower_sessions::Expiry`
- Konfiguriere Session-Cookie mit:
  - `same_site(SameSite::Lax)` - erlaubt Cookie bei OAuth-Redirects
  - `secure(true)` für HTTPS (via env var `SESSION_SECURE`, default: false für localhost)
  - `http_only(true)` - verhindert XSS-Angriffe
  - `path("/")` - Cookie gilt für alle Routen
  - `max_age(Duration::days(30))` - Cookie bleibt 30 Tage gültig

**Code:**
```rust
use tower_sessions::{MemoryStore, Session, SessionManagerLayer, Expiry, cookie::{Cookie, SameSite}};
use std::time::Duration;

pub async fn create_app(db_provider: Arc<dyn DatabaseProvider>) -> Router {
    let session_store = MemoryStore::default();
    
    // Configure session cookie
    let session_secure = std::env::var("SESSION_SECURE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(session_secure)
        .with_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_path("/")
        .with_max_age(Duration::from_secs(30 * 24 * 60 * 60)) // 30 days
        .with_expiry(Expiry::OnInactivity(Duration::from_secs(30 * 24 * 60 * 60)));
    
    // ... rest of the code
}
```

### 2. Debug-Logging in `12_auth.rs`

**Datei:** `src/12_auth.rs`

**Änderungen im `auth_callback` Handler (nach Zeile 145):**

```rust
match state.db_provider.upsert_candidate(&candidate).await {
    Ok(candidate_id) => {
        // Debug: Log session ID before insert
        println!("DEBUG: Session ID before insert: {:?}", session.id());
        
        // Store user_id in session to maintain login state
        session.insert("user_id", candidate_id).await.unwrap();
        session.insert("user_name", name.clone()).await.unwrap();
        
        // Debug: Log session ID after insert and verify values
        println!("DEBUG: Session ID after insert: {:?}", session.id());
        println!("DEBUG: Stored user_id: {}, user_name: {}", candidate_id, name);
        
        // Force session save
        if let Err(e) = session.save().await {
            eprintln!("ERROR: Failed to save session: {:?}", e);
        }
        
        Redirect::to("/")
    }
    // ...
}
```

**Änderungen im `login` Handler (nach Zeile 61):**

```rust
// Store CSRF token in session for validation
session
    .insert("oauth_csrf", csrf_token.secret())
    .await
    .unwrap();

// Debug: Verify CSRF token was stored
println!("DEBUG login: Session ID: {:?}, stored CSRF token", session.id());

Redirect::to(auth_url.as_ref())
```

### 3. Debug-Logging in `11_web_server.rs`

**Datei:** `src/11_web_server.rs`

**Änderungen in `root` Handler (Zeile 33):**

```rust
async fn root(session: Session) -> Html<String> {
    // Debug: Log session state
    println!("DEBUG root: Session ID: {:?}", session.id());
    
    if let Some(user_name) = session.get::<String>("user_name").await.unwrap() {
        let user_id = session.get::<i32>("user_id").await.unwrap();
        println!("DEBUG root: Found user - ID: {:?}, Name: {}", user_id, user_name);
        
        Html(format!(
            r#"Hello {}! Welcome to Roche Job Scraper.
            // ... rest
        ))
    } else {
        println!("DEBUG root: No user found in session");
        Html(
            r#"Hello from Roche Job Scraper Web Server!
            // ... rest
        )
    }
}
```

### 4. Test-Strategie

**Manuelle Tests:**

1. **Starte Server**: `cargo run --bin stage10_web --features web`
2. **Öffne Browser DevTools** → Application/Storage → Cookies → `http://localhost:3000`
3. **Klicke "Login with GitHub"**:
   - Prüfe: Session-Cookie wird gesetzt
   - Prüfe Terminal: "DEBUG login: Session ID: ..." erscheint
4. **Nach GitHub-Auth-Flow und Callback**:
   - Prüfe: Gleicher Session-Cookie noch vorhanden?
   - Prüfe Terminal: "DEBUG: Session ID before/after insert" erscheint
   - Prüfe: Session-ID ist identisch vor und nach insert?
5. **Auf Hauptseite**:
   - Prüfe Terminal: "DEBUG root: Found user - ID: ..." erscheint
   - Prüfe Browser: "Hello {Name}!" wird angezeigt

**Cookie-Eigenschaften prüfen:**
- `SameSite`: Lax
- `Path`: /
- `HttpOnly`: true
- `Secure`: false (localhost) oder true (HTTPS)
- `Max-Age` oder `Expires`: ~30 Tage

### 5. CSRF-Validierung reaktivieren

**Datei:** `src/12_auth.rs`

**Nach erfolgreichen Tests der Session-Persistenz:**

Kommentiere Zeilen 77-86 aus und entferne den TODO-Kommentar:

```rust
async fn auth_callback(
    Extension(state): Extension<AuthState>,
    Query(params): Query<AuthCallback>,
    session: Session,
) -> impl IntoResponse {
    // Validate CSRF token
    if let Some(stored_csrf) = session.get::<String>("oauth_csrf").await.unwrap() {
        if stored_csrf != params.state {
            eprintln!("CSRF validation failed: expected {}, got {}", stored_csrf, params.state);
            return Redirect::to("/?error=csrf");
        }
        // Clear the CSRF token from session after successful validation
        let _ = session.remove::<String>("oauth_csrf").await;
    } else {
        eprintln!("No CSRF token found in session");
        return Redirect::to("/?error=no_csrf");
    }

    // Exchange code for token
    // ...
}
```

**WICHTIG:** Die `AuthCallback` struct muss das `state` Feld enthalten:

```rust
#[derive(Deserialize)]
struct AuthCallback {
    code: String,
    state: String,  // CSRF token from OAuth provider
}
```

## Production-Überlegungen

### 1. HTTPS/Nginx-Konfiguration

Für Production mit Nginx reverse proxy:

**Umgebungsvariablen setzen:**
```bash
export SESSION_SECURE=true
export SESSION_DOMAIN=your-domain.com
```

**Cookie-Konfiguration erweitern in `11_web_server.rs`:**
```rust
let session_domain = std::env::var("SESSION_DOMAIN").ok();

let mut session_layer = SessionManagerLayer::new(session_store)
    .with_secure(session_secure)
    .with_same_site(SameSite::Lax)
    .with_http_only(true)
    .with_path("/")
    .with_max_age(Duration::from_secs(30 * 24 * 60 * 60))
    .with_expiry(Expiry::OnInactivity(Duration::from_secs(30 * 24 * 60 * 60)));

if let Some(domain) = session_domain {
    session_layer = session_layer.with_domain(domain);
}
```

### 2. SQLite Session-Store (Optional)

Für Production mit Server-Restarts oder Load-Balancing:

**Dependency in `Cargo.toml` ergänzen:**
```toml
tower-sessions-sqlx-store = { version = "0.15", features = ["sqlite"], optional = true }
```

**Code in `11_web_server.rs` ändern:**
```rust
use tower_sessions_sqlx_store::{SqliteStore, sqlx::SqlitePool};

pub async fn create_app(db_provider: Arc<dyn DatabaseProvider>) -> Router {
    // Create SQLite pool for sessions
    let pool = SqlitePool::connect("sqlite:sessions.db").await?;
    let session_store = SqliteStore::new(pool);
    session_store.migrate().await?;
    
    let session_layer = SessionManagerLayer::new(session_store)
        // ... same configuration as above
    
    // ...
}
```

### 3. Konfigurierbare Session-Lifetime

**Umgebungsvariable:** `SESSION_MAX_AGE_DAYS` (Default: 30)

```rust
let session_max_age_days = std::env::var("SESSION_MAX_AGE_DAYS")
    .unwrap_or_else(|_| "30".to_string())
    .parse::<u64>()
    .unwrap_or(30);

let session_max_age_secs = session_max_age_days * 24 * 60 * 60;

let session_layer = SessionManagerLayer::new(session_store)
    .with_max_age(Duration::from_secs(session_max_age_secs))
    .with_expiry(Expiry::OnInactivity(Duration::from_secs(session_max_age_secs)));
```

## Erfolgs-Kriterien

- [ ] Session-Cookie wird beim Login gesetzt
- [ ] Session-Cookie überlebt OAuth-Redirect (GitHub → Callback)
- [ ] Session-ID bleibt identisch während des gesamten Flows
- [ ] Nach erfolgreichem Login zeigt Root-Route "Hello {Name}!"
- [ ] Session bleibt nach Browser-Refresh erhalten
- [ ] CSRF-Validierung funktioniert nach Aktivierung
- [ ] Logout löscht Session vollständig

## Bekannte Limitierungen

1. **MemoryStore**: Sessions gehen bei Server-Restart verloren
2. **Localhost-Testing**: `secure: false` erforderlich für HTTP-Testing
3. **Single-Server**: MemoryStore funktioniert nicht mit Load-Balancing

## Nächste Schritte nach Behebung

1. Session-basierte Features implementieren:
   - Dashboard mit personalisierten Job-Matches
   - Profil-Editor für `profile_text`
   - Job-Favoriten/Bookmarks

2. Session-Store auf SQLite/PostgreSQL upgraden für Production

3. Rate-Limiting pro Session implementieren

