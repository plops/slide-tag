# Implementierungsplan: E2E Integrationstests (Schritt 15)

Wir implementieren nun einen automatisierten End-to-End (E2E) Test für die komplette Web-Anwendung. Wir bleiben vollständig in **Rust** und nutzen die bestehende **`chromiumoxide`** Library, um den Headless Browser zu steuern.

## 1. Deepwiki MCP Kontext (WICHTIG für die KI)
Wenn du Dokumentation benötigst, nutze Deepwiki MCP mit folgenden Repositories:
*   Axum Webserver: `tokio-rs/axum`
*   Chromiumoxide: `mattsse/chromiumoxide`
*   Tower Sessions: `maxcountryman/tower-sessions`
*   Das eigene Projekt heißt: `plops/slide-tag` (darin der Unterordner `companies/roche/rs_scrape_jobs`).

## 2. Vorbereitung: Die Dev-Login Backdoor
Echte GitHub-Logins schlagen im automatisierten Test wegen Bot-Protection fehl. Wir brauchen einen Mock-Login.

**Aufgabe:**
1.  Gehe in `src/12_auth.rs` und füge eine neue Funktion `async fn dev_mock_login(...)` hinzu.
2.  Diese Route soll **nur** aktiv sein, wenn `config.is_debug == true`.
3.  Die Route soll einen Fake-User erstellen (Name: exakt der aus `config.admin_username`, also z.B. "plops", `oauth_sub`: "dev_test_sub"). Schreibe ihn via `db_provider.upsert_candidate` in die DB, setze die Werte in die `Session` (analog zum normalen Login) und mache einen Redirect auf `/dashboard`.
4.  Füge diese Route in `11_web_server.rs` dem Router hinzu (z.B. unter `/auth/dev-login`).

## 3. Vorbereitung: Ein Mock für die KI
Damit E2E-Tests nicht echtes Geld über die Gemini-API kosten und schnell durchlaufen, erstelle in `src/07_ai_core.rs` (oder einer speziellen Test-Datei) einen `MockAiProvider`, der das `AiProvider` Trait implementiert. Er soll einfach statische Dummy-Matches und Dummy-Annotations zurückgeben.

## 4. Implementierung des E2E-Tests
Erstelle eine neue Datei `tests/web_integration_e2e.rs`.
Nutze `tokio::test`. 

### 4.1 Setup (Test-Infrastruktur hochfahren)
*   Initialisiere eine **temporäre SQLite Datenbank** (z.B. eine Datei `test_e2e.db`, die am Ende gelöscht wird).
*   Füge manuell 2-3 Dummy-Jobs in die Datenbank ein, damit wir etwas sehen.
*   Erstelle eine `AppConfig` mit `is_debug = true`, `port = 3030`, `admin_username = "plops"`.
*   Instanziiere `AppState` (nutze hier den `MockAiProvider` statt dem `GeminiProvider`).
*   Starte den Webserver asynchron in einem Hintergrundtask:
    ```rust
    tokio::spawn(async move {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3030));
        web_server::run_server(addr, app_state, &config).await.unwrap();
    });
    // Warte kurz, bis der Server lauscht
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    ```
*   Starte Chromiumoxide:
    ```rust
    use chromiumoxide::browser::{Browser, BrowserConfig};
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder().with_head().build().unwrap() // .with_head() ggf. entfernen für reines headless
    ).await.unwrap();
    let handler_task = tokio::spawn(async move {
        use futures::StreamExt;
        while let Some(_) = handler.next().await {}
    });
    let page = browser.new_page("about:blank").await.unwrap();
    ```

### 4.2 Die Workflows testen (Chronologischer Ablauf)
Steuere die `page` nun programmatisch durch alle Workflows. Da `chromiumoxide` kein "Auto-Waiting" hat, musst du nach jedem Button-Klick oder `goto` kurz warten (`tokio::time::sleep`), damit das DOM rendern kann.

**Schritt A: Ohne Login browsen**
*   Gehe zu `http://localhost:3030/jobs`: `page.goto(...).await;`
*   Lies das DOM aus und `assert!`, dass die Dummy-Jobs sichtbar sind (z.B. Textsuche nach dem Titel).

**Schritt B: Einloggen**
*   Gehe zu `http://localhost:3030/auth/dev-login`.
*   Durch den Redirect solltest du nun auf `/dashboard` landen. Lese das DOM aus und prüfe, ob der Text "Welcome" oder "plops" (User-Name) auf dem Dashboard steht.

**Schritt C: Profil eingeben**
*   Gehe zu `http://localhost:3030/profile`.
*   Fülle das Textfeld via JavaScript aus:
    `page.evaluate("document.querySelector('#profile_text').value = 'Senior Rust Architect';").await;`
*   Submitte das Formular:
    `page.evaluate("document.querySelector('form').submit();").await;`
*   Warte 1-2 Sekunden. Prüfe, ob du auf dem Dashboard landest und das Profil gespeichert wurde.

**Schritt D: Matches generieren**
*   Auf dem Dashboard, klicke den "Re-evaluate Matches" Button:
    `page.evaluate("document.querySelector('form[action=\"/api/trigger-match\"] button').click();").await;`
*   (Da dies im Backend asynchron läuft, warte etwas und lade das Dashboard via `page.goto` neu).
*   Prüfe, ob jetzt ein Match (vom MockAiProvider) in der Liste auftaucht.

**Schritt E: Match Details aufrufen**
*   Lies via JS die URL des ersten Match-Links aus:
    `let url = page.evaluate("document.querySelector('.match-card a').href").await?.into_value::<String>()?;`
*   Navigiere dorthin: `page.goto(&url).await;`
*   Prüfe, ob die Seite den Titel des Jobs und die KI-Erklärung anzeigt.

**Schritt F: Admin Scraping triggern**
*   Gehe zu `http://localhost:3030/admin`.
*   (Funktioniert nur, weil unser Dev-Login-User "plops" ist!)
*   Klicke auf den Button:
    `page.evaluate("document.querySelector('form[action=\"/admin/trigger\"] button').click();").await;`
*   Prüfe, ob sich der Status auf dem UI auf "Running" oder "Success" ändert.

### 4.3 Cleanup
Am Ende der Test-Funktion:
```rust
browser.close().await.unwrap();
handler_task.await.unwrap();
// Lösche die temporäre test_e2e.db
```

**Anweisung für Code-Qualität:** 
Setze den Test-Code in kleine Hilfsfunktionen (z.B. `async fn wait_and_click(page: &Page, selector: &str)`), um den Haupttest lesbar und robust zu halten. Achte peinlich genau darauf, dass bei asynchronen Aufrufen keine Panics auftreten, die den Test-Runner einfrieren.
