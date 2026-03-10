# Gesamter Implementierungsplan: E2E-Tests mit Playwright-ähnlichem Auto-Waiting in Rust

Als Vorbereitung für stabile End-to-End Tests implementieren wir zunächst eine eigene Auto-Waiting-Bibliothek (als separaten Crate) auf Basis von `chromiumoxide`, die sich an der Architektur von Playwright orientiert. Anschließend schreiben wir die eigentlichen E2E-Tests für unsere Applikation unter Nutzung dieser neuen Bibliothek.

## TEIL 1: Architektur & Implementierung des `chromiumoxide_autowait` Crates

Wir wandeln das Projekt in einen Cargo-Workspace um (falls noch nicht geschehen) und erstellen einen neuen lokalen Crate. Das Ziel ist eine ergonomische API mittels *Extension Traits*, sodass wir im Test einfach `page.auto_click("#submit-btn").await` aufrufen können und der Crate intern Visibility, Stabilität und Klickbarkeit prüft.

### 1.1 Workspace Setup
**Aufgabe:** 
Füge in der Haupt-`Cargo.toml` (im Root von `rs_scrape_jobs`) am Ende folgendes hinzu:
```toml
[workspace]
members =[
    ".",
    "crates/chromiumoxide_autowait"
]
```
Füge außerdem in der Haupt-`Cargo.toml` unter `[dependencies]` hinzu:
```toml
chromiumoxide_autowait = { path = "crates/chromiumoxide_autowait" }
```

### 1.2 Struktur des neuen Crates erstellen
**Aufgabe:**
Erstelle das Verzeichnis `crates/chromiumoxide_autowait` und darin eine `Cargo.toml` sowie den `src/`-Ordner.

**`crates/chromiumoxide_autowait/Cargo.toml`**:
```toml
[package]
name = "chromiumoxide_autowait"
version = "0.1.0"
edition = "2021"

[dependencies]
chromiumoxide = "0.9"
tokio = { version = "1", features = ["time"] }
thiserror = "1.0"
futures = "0.3"
```

### 1.3 Core-Typen definieren (`src/types.rs`)
Definiere die Zustände, auf die wir warten, und die Fehlerarten.

```rust
// crates/chromiumoxide_autowait/src/types.rs
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementState {
    Visible,
    Stable,
    Enabled,
    Editable,
}

#[derive(Debug, Error)]
pub enum ActionabilityError {
    #[error("Element state missing: {0:?}")]
    MissingState(ElementState),
    #[error("Element is not connected to the DOM")]
    NotConnected,
    #[error("Timeout reached while waiting for actionability")]
    Timeout,
    #[error("Chromiumoxide protocol error: {0}")]
    ProtocolError(String),
}

pub struct AutoWaitOptions {
    pub timeout: Duration,
}

impl Default for AutoWaitOptions {
    fn default() -> Self {
        Self { timeout: Duration::from_secs(10) } // Default 10s timeout
    }
}
```

### 1.4 JavaScript-Injektionen (`src/scripts.rs`)
Hier liegen die JS-Snippets, die im Kontext der Seite ausgeführt werden (basierend auf der Playwright-Logik).

```rust
// crates/chromiumoxide_autowait/src/scripts.rs

pub const CHECK_STATES_JS: &str = r#"
(function(selector, states) {
    const el = document.querySelector(selector);
    if (!el) return { error: 'notconnected' };

    const rect = el.getBoundingClientRect();

    for (const state of states) {
        if (state === 'visible') {
            if (rect.width === 0 || rect.height === 0) return { missingState: 'visible' };
            const style = window.getComputedStyle(el);
            if (style.visibility === 'hidden') return { missingState: 'visible' };
        }
        if (state === 'enabled') {
            if (el.disabled || el.closest('[aria-disabled=true]'))
                return { missingState: 'enabled' };
        }
        if (state === 'editable') {
            if (el.disabled || el.readOnly) return { missingState: 'editable' };
        }
    }
    return { ok: true };
})
"#;

pub const CHECK_STABLE_JS: &str = r#"
(function(selector) {
    return new Promise((resolve) => {
        const el = document.querySelector(selector);
        if (!el) { resolve({ error: 'notconnected' }); return; }
        
        let lastRect = el.getBoundingClientRect();
        requestAnimationFrame(() => {
            const newRect = el.getBoundingClientRect();
            const stable = lastRect.x === newRect.x && lastRect.y === newRect.y
                        && lastRect.width === newRect.width && lastRect.height === newRect.height;
            resolve(stable ? { ok: true } : { missingState: 'stable' });
        });
    });
})
"#;
```

### 1.5 Die Retry-Loop (Die Engine) (`src/waiter.rs`)
Implementiere die Polling-Schleife mit dem progressiven Backoff.

```rust
// crates/chromiumoxide_autowait/src/waiter.rs
use crate::types::{ActionabilityError, AutoWaitOptions, ElementState};
use crate::scripts::{CHECK_STATES_JS, CHECK_STABLE_JS};
use chromiumoxide::Page;
use std::time::Instant;
use tokio::time::sleep;

const BACKOFF_DELAYS_MS: &[u64] = &[0, 20, 100, 100, 500];

pub async fn wait_for_states(
    page: &Page,
    selector: &str,
    states: &[ElementState],
    options: &AutoWaitOptions,
) -> Result<(), ActionabilityError> {
    let deadline = Instant::now() + options.timeout;
    let mut retry = 0usize;

    loop {
        if Instant::now() >= deadline {
            return Err(ActionabilityError::Timeout);
        }

        if retry > 0 {
            let delay_ms = BACKOFF_DELAYS_MS[retry.min(BACKOFF_DELAYS_MS.len() - 1)];
            sleep(std::time::Duration::from_millis(delay_ms)).await;
        }

        let mut all_passed = true;
        
        // Erst Stabilität prüfen (Async via requestAnimationFrame)
        if states.contains(&ElementState::Stable) {
            let js = format!("({CHECK_STABLE_JS})(`{selector}`)");
            match page.evaluate(js).await {
                Ok(res) => {
                    let val = res.into_value::<serde_json::Value>().unwrap_or_default();
                    if val.get("error").is_some() || val.get("missingState").is_some() {
                        all_passed = false;
                    }
                },
                Err(_) => { all_passed = false; }
            }
        }

        // Dann synchrone States prüfen
        if all_passed {
            let sync_states: Vec<&str> = states.iter()
                .filter(|&&s| s != ElementState::Stable)
                .map(|s| match s {
                    ElementState::Visible => "visible",
                    ElementState::Enabled => "enabled",
                    ElementState::Editable => "editable",
                    _ => "",
                })
                .collect();
            
            if !sync_states.is_empty() {
                let states_json = serde_json::to_string(&sync_states).unwrap();
                let js = format!("({CHECK_STATES_JS})(`{selector}`, {states_json})");
                match page.evaluate(js).await {
                    Ok(res) => {
                        let val = res.into_value::<serde_json::Value>().unwrap_or_default();
                        if val.get("error").is_some() || val.get("missingState").is_some() {
                            all_passed = false;
                        }
                    },
                    Err(_) => { all_passed = false; }
                }
            }
        }

        if all_passed { return Ok(()); }
        retry += 1;
    }
}
```

### 1.6 Extension Trait API (`src/ext.rs` und `src/lib.rs`)
Binde die Logik elegant an `chromiumoxide::Page`.

```rust
// crates/chromiumoxide_autowait/src/ext.rs
use crate::types::{ActionabilityError, AutoWaitOptions, ElementState};
use crate::waiter::wait_for_states;
use chromiumoxide::Page;
use std::future::Future;
use std::pin::Pin;

pub trait PageAutoWaitExt {
    fn auto_click<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>>;
    fn auto_fill<'a>(&'a self, selector: &'a str, text: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>>;
    fn auto_wait_visible<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>>;
}

impl PageAutoWaitExt for Page {
    fn auto_click<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>> {
        Box::pin(async move {
            let opts = AutoWaitOptions::default();
            let states =[ElementState::Visible, ElementState::Stable, ElementState::Enabled];
            wait_for_states(self, selector, &states, &opts).await?;
            
            let el = self.find_element(selector).await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            el.click().await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            Ok(())
        })
    }

    fn auto_fill<'a>(&'a self, selector: &'a str, text: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>> {
        Box::pin(async move {
            let opts = AutoWaitOptions::default();
            // Fill braucht Visible, Enabled, Editable
            let states =[ElementState::Visible, ElementState::Enabled, ElementState::Editable];
            wait_for_states(self, selector, &states, &opts).await?;
            
            let el = self.find_element(selector).await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            // Click to focus, then clear and type
            el.click().await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            self.evaluate(format!("document.querySelector(`{selector}`).value = ''")).await.ok();
            el.type_text(text).await.map_err(|e| ActionabilityError::ProtocolError(e.to_string()))?;
            Ok(())
        })
    }

    fn auto_wait_visible<'a>(&'a self, selector: &'a str) -> Pin<Box<dyn Future<Output = Result<(), ActionabilityError>> + 'a>> {
        Box::pin(async move {
            let opts = AutoWaitOptions::default();
            wait_for_states(self, selector, &[ElementState::Visible], &opts).await
        })
    }
}
```

```rust
// crates/chromiumoxide_autowait/src/lib.rs
pub mod types;
pub mod scripts;
pub mod waiter;
pub mod ext;

pub use types::*;
pub use ext::PageAutoWaitExt;
```

---

## TEIL 2: E2E Integrationstests (Die App testen)

Nun schreiben wir einen eleganten E2E-Test in `rs_scrape_jobs/tests/web_integration_e2e.rs`, der keine schmutzigen `sleep` Hacks mehr benötigt.

Wenn du Dokumentation benötigst, nutze Deepwiki MCP mit folgenden Repositories:
*   Axum Webserver: `tokio-rs/axum`
*   Chromiumoxide: `mattsse/chromiumoxide`
*   Tower Sessions: `maxcountryman/tower-sessions`
*   Das eigene Projekt heißt: `plops/slide-tag` (darin der Unterordner `companies/roche/rs_scrape_jobs`).

### 2.1 Vorbereitung: Dev-Login Backdoor & AI Mock
Genau wie im alten Plan brauchen wir eine Mock-Login-Backdoor und einen MockAiProvider, damit wir ohne reelles GitHub OAuth und ohne kostenpflichtige Gemini-Calls testen können.

**Aufgaben:**
1.  Füge in `src/12_auth.rs` eine Route `dev_mock_login` hinzu, die nur verfügbar ist, wenn `config.is_debug == true`. Sie generiert einen Dummy-User und schreibt ihn als eingeloggt in die `Session`.
2.  Erstelle in den Tests (`tests/web_integration_e2e.rs`) einen struct `MockAiProvider`, das `AiProvider` implementiert und sofort Dummydaten (`CandidateMatch`) zurückliefert.

### 2.2 Der saubere E2E-Test (`tests/web_integration_e2e.rs`)

Nutze den neuen `PageAutoWaitExt` Trait für alle Interaktionen.

```rust
use rs_scrape::{app_state::AppState, config::AppConfig, web_server};
use chromiumoxide_autowait::PageAutoWaitExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::sync::Arc;

// (Hier Code für MockAiProvider einfügen)

#[tokio::test]
async fn test_full_user_journey() {
    // 1. Setup DB & AppState mit MockAiProvider
    // ... (wie gehabt: test_e2e.db anlegen, 2 Dummy Jobs injecten)
    
    // 2. Server im Hintergrund starten (z.B. Port 3040)
    let port = 3040;
    // ... tokio::spawn(web_server::run_server(...));
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // 3. Browser starten
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder().build().unwrap()
    ).await.unwrap();
    let handler_task = tokio::spawn(async move {
        while let Some(_) = handler.next().await {}
    });
    let page = browser.new_page("about:blank").await.unwrap();

    // SCHRITT A: Ohne Login Jobs ansehen
    page.goto(format!("http://localhost:{}/jobs", port)).await.unwrap();
    // Auto-wait for the job card to appear! Kein manuelles Sleep mehr!
    page.auto_wait_visible(".job-card").await.expect("Job card did not appear");

    // SCHRITT B: Dev-Login
    page.goto(format!("http://localhost:{}/auth/dev-login", port)).await.unwrap();
    // Wir sollten direkt aufs Dashboard redirected werden. Wir warten bis die Stats laden:
    page.auto_wait_visible(".stat-card").await.expect("Dashboard failed to load");

    // SCHRITT C: Profil bearbeiten
    page.goto(format!("http://localhost:{}/profile", port)).await.unwrap();
    // Nutze auto_fill (wartet auf Visible, Enabled, Editable)
    page.auto_fill("#profile_text", "Senior Rust Engineer with AI experience").await.unwrap();
    // Nutze auto_click (wartet auf Visible, Enabled, Stable)
    page.auto_click("button[type=\"submit\"]").await.unwrap();

    // SCHRITT D: Match triggern
    page.auto_wait_visible(".alert-success").await.expect("Success alert missing"); // Optional, falls Profil-Save auf der Seite bleibt
    page.goto(format!("http://localhost:{}/dashboard", port)).await.unwrap();
    
    // Auto-click auf den Re-evaluate Button
    page.auto_click("form[action=\"/api/trigger-match\"] button").await.unwrap();
    
    // Nach kurzem Reload (da Background Job):
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    page.goto(format!("http://localhost:{}/dashboard", port)).await.unwrap();
    
    // Warten, bis mindestens eine Match-Card gerendert ist
    page.auto_wait_visible(".match-card").await.expect("Matches were not generated");

    // SCHRITT E: Admin Funktion testen
    page.goto(format!("http://localhost:{}/admin", port)).await.unwrap();
    // Warten bis Admin Button bereit ist, dann klicken
    page.auto_click("form[action=\"/admin/trigger\"] button").await.unwrap();

    // Cleanup
    browser.close().await.unwrap();
    handler_task.await.unwrap();
}
```

### Zusammenfassung der Anweisungen an die KI:
Bitte setze dieses Setup exakt wie beschrieben um.
1. Erstelle zuerst den Workspace und den `chromiumoxide_autowait` Crate.
2. Implementiere die Extension Traits.
3. Füge die `dev_mock_login` Route in der Haupt-Applikation (`rs_scrape_jobs`) ein.
4. Schreibe den E2E-Test, der die volle Power des neuen `auto_wait` Crates demonstriert. Achte auf saubere Fehlerbehandlung (verwende `expect` mit aussagekräftigen Fehlermeldungen in den Tests).
