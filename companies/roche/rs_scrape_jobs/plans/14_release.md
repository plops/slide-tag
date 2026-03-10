

Du nimmst die Rolle eines Senior Rust Developers ein. Führe die folgenden Änderungen strikt und präzise durch.

Anweisung A:

### 1. Bugfixes für Deployment & Architektur

**Aktion 1.1: Binary Name in `Cargo.toml` anpassen**
Ändere in der `Cargo.toml` den Namen des Binaries von `main` zu `rs-scrape`.
```toml
[[bin]]
name = "rs-scrape"
path = "src/bin/main.rs"
```

**Aktion 1.2: `08_config.rs` erweitern**
Füge die fehlenden OAuth- und Session-Variablen in `AppConfig` hinzu.
```rust
// In src/08_config.rs
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_path: String,
    pub gemini_api_key: String,
    pub host: String,
    pub port: u16,
    pub is_debug: bool,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub oauth_redirect_url: String,
    pub session_secure: bool,
    pub session_max_age_days: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            db_path: env::var("DB_PATH").unwrap_or_else(|_| "jobs_minutils.db".to_string()),
            gemini_api_key: env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY muss gesetzt sein"),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap_or(3000),
            is_debug: env::var("DEBUG").unwrap_or_else(|_| "false".to_string()).parse().unwrap_or(false),
            github_client_id: env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID muss gesetzt sein"),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET muss gesetzt sein"),
            oauth_redirect_url: env::var("OAUTH_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
            session_secure: env::var("SESSION_SECURE").unwrap_or_else(|_| "false".to_string()).parse().unwrap_or(false),
            session_max_age_days: env::var("SESSION_MAX_AGE_DAYS").unwrap_or_else(|_| "30".to_string()).parse().unwrap_or(30),
        }
    }
}
```

**Aktion 1.3: `11_web_server.rs` bereinigen**
Lösche die direkten `std::env::var` Aufrufe in `create_app` und nutze stattdessen die Werte aus der Config.
*Hinweis:* Ändere die Signatur von `create_app` zu `pub async fn create_app(app_state: Arc<AppState>, config: &AppConfig) -> Router`. Aktualisiere auch `run_server` entsprechend, damit es die `AppConfig` durchreicht.

### 2. Server Konfiguration sanitizen (Verschleierung sensibler Daten)

Um die Serverstruktur im Git zu haben, ohne Pfade zu leaken, erstellen wir Template-Dateien.
**Aktion:** Lösche den Ordner `config_release/` und erstelle einen neuen Ordner `deployment/templates/`.

Erstelle `deployment/templates/rs-scrape.service`:
```ini
[Unit]
Description=RocketRecap Jobs Rust App
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/DEPLOY_PATH
ExecStart=/DEPLOY_PATH/rs-scrape serve --port 3000
Restart=always
Environment="PORT=3000"
EnvironmentFile=/DEPLOY_PATH/.env[Install]
WantedBy=multi-user.target
```

Erstelle `deployment/templates/nginx.conf.snippet`:
```nginx
# Nginx Konfiguration für die Rust-App
server {
    listen 443 ssl;
    listen [::]:443 ssl;
    server_name YOUR_DOMAIN;

    ssl_certificate      /etc/letsencrypt/live/YOUR_DOMAIN/fullchain.pem;
    ssl_certificate_key  /etc/letsencrypt/live/YOUR_DOMAIN/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 3. GitHub Workflow anpassen (`release.yml`)

Der alte Workflow nutzt `zipsign`, was für dieses Server-Deployment unnötig komplex ist. Wir vereinfachen ihn auf ein sauberes Release-Build für Linux (Hetzner).

**Aktion:** Ersetze den Inhalt von `.github/workflows/release.yml` komplett mit folgendem Code:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-release:
    name: Build and Release
    runs-on: ubuntu-latest
    permissions:
      contents: write
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-unknown-linux-gnu

      - name: Rust Cache
        uses: Swatinem/rust-cache@v2

      - name: Build release
        run: cargo build --release --target x86_64-unknown-linux-gnu

      - name: Prepare Asset
        run: |
          tar czf rs-scrape-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release rs-scrape

      - name: Upload assets to release
        uses: softprops/action-gh-release@v2
        if: startsWith(github.ref, 'refs/tags/')
        with:
          files: rs-scrape-linux-x86_64.tar.gz
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 4. Dokumentation aktualisieren

**Aktion:** Aktualisiere `docs/release_process.md` vollständig:
```markdown
# Release Process

Dieses Projekt nutzt GitHub Actions, um automatisch Release-Binaries für Linux (x86_64) zu bauen. Dies ist ideal für das Deployment auf Hetzner Ubuntu-Servern.

## Automatischer Release

Releases werden getriggert, sobald ein Git-Tag gepusht wird, das mit `v` beginnt (z.B. `v1.0.0`).

### Schritt-für-Schritt Anleitung

1.  **Version anpassen**:
    Aktualisiere die Version in der `Cargo.toml`.
    ```toml
    [package]
    name = "rs-scrape"
    version = "1.0.0"
    ```

2.  **Commit und Push**:
    ```bash
    git add .
    git commit -m "Prepare release v1.0.0"
    git push origin main
    ```

3.  **Tag erstellen und pushen**:
    ```bash
    git tag v1.0.0
    git push origin v1.0.0
    ```

4.  **Download Asset**:
    Unter dem Reiter **Releases** auf GitHub erscheint nach wenigen Minuten die Datei `rs-scrape-linux-x86_64.tar.gz`. Diese kann direkt auf den Hetzner Server heruntergeladen und entpackt werden.
```

Anweisung B:


Du nimmst die Rolle eines Senior Rust Developers ein. Deine Aufgabe ist es, das unkontrollierte Schreiben von Debug-HTML-Dateien in der Produktionsumgebung zu unterbinden.

Aktuell schreibt `src/03_scraper_roche.rs` bei jedem Paginierungs-Schritt hartcodiert Dateien auf die Festplatte. Wir haben bereits einen `debug_dump` Flag in unserer Pipeline, diesen müssen wir nun bis zum Scraper durchreichen.

Führe folgende Änderungen präzise durch:

### 1. `src/03_scraper_roche.rs` anpassen

Ändere die Signaturen von `collect_job_urls` und `scrape_roche_jobs`, damit sie den `debug_dump: bool` Parameter akzeptieren. Umschließe die `fs::write` Aufrufe mit einer `if debug_dump` Bedingung.

```rust
// In src/03_scraper_roche.rs

// ... (Imports bleiben gleich)

/// Collect job URLs by paginating through results
pub async fn collect_job_urls(page: &Page, debug_dump: bool) -> Result<Vec<String>> {
    println!("Collecting job URLs from paginated results...");
    let mut links = HashSet::new();
    let mut visited_urls = HashSet::new();
    let timeout = Duration::from_secs(60);
    let start = Instant::now();

    loop {
        if start.elapsed() > timeout {
            break;
        }

        let current_url = page.url().await?;
        if visited_urls.contains(&current_url) {
            println!("Re-visiting URL, breaking loop.");
            break;
        }
        visited_urls.insert(current_url.clone());

        let result = page.evaluate(r#"
            Array.from(document.querySelectorAll("a[data-ph-at-id='job-link']")).map(a => a.href).filter(href => href)
        "#).await?;
        let hrefs: Vec<String> = serde_json::from_value(result.value().unwrap().clone())?;
        let _prev_count = links.len();
        for href in hrefs {
            links.insert(href.clone());
        }

        // NUR schreiben, wenn debug_dump aktiv ist
        if debug_dump {
            let html_before = page.content().await?;
            let _ = fs::write("page_before_click.html", &html_before);
            println!("Dumped HTML to page_before_click.html");
        }

        let next_result = page
            .evaluate(r#"
            let nextBtn = document.querySelector('a[data-ph-at-id="pagination-next-link"]');
            if (nextBtn && nextBtn.href && typeof nextBtn.href === 'string' && nextBtn.href.startsWith('https')) {
                nextBtn.href
            } else {
                null
            }
            "#)
            .await?;
            
        if let Some(val) = next_result.value() {
            if let Some(href) = val.as_str() {
                println!("Navigating to next page: {}", href);
                page.goto(href).await?;
                page.wait_for_navigation().await?;
                println!(
                    "Navigated to: {}",
                    page.url().await?.unwrap_or_else(|| "No URL".to_string())
                );
                
                // NUR schreiben, wenn debug_dump aktiv ist
                if debug_dump {
                    let html_after = page.content().await?;
                    let _ = fs::write("page_after_click.html", &html_after);
                    println!("Dumped HTML to page_after_click.html");
                }
            } else {
                println!("No valid next button href, last page.");
                break;
            }
        } else {
            println!("Evaluate failed, breaking.");
            break;
        }
    }

    let mut sorted_links: Vec<String> = links.into_iter().collect();
    sorted_links.sort();
    println!("Collected {} job links.", sorted_links.len());
    Ok(sorted_links)
}

/// Main scraper function
pub async fn scrape_roche_jobs(page: &Page, debug_dump: bool) -> Result<Vec<String>> {
    navigate_to_roche(page).await?;
    handle_cookie_banner(page).await?;
    click_ort_accordion(page).await?;
    enter_schweiz_filter(page).await?;
    click_schweiz_checkbox(page).await?;
    collect_job_urls(page, debug_dump).await
}
```

### 2. `src/06b_pipeline_orchestrator.rs` anpassen

Aktualisiere den Aufruf von `scrape_roche_jobs`, damit das `debug_dump` Flag korrekt übergeben wird.

```rust
// In src/06b_pipeline_orchestrator.rs
// ... (Imports bleiben gleich)

pub async fn run_pipeline(repo: &JobRepository, debug_dump: bool) -> Result<()> {
    // Setup browser
    let (mut browser, page, handle) = web_core::setup_browser().await?;

    // 1. Scrape URLs (Jetzt mit debug_dump Parameter!)
    let urls = scraper_roche::scrape_roche_jobs(&page, debug_dump).await?;

    // Cleanup browser
    browser.close().await?;
    handle.await?;

    // ... (Rest der Datei bleibt unverändert)
```

### 3. `src/bin/stage3_search.rs` reparieren (falls vorhanden)
Da sich die Signatur von `scrape_roche_jobs` geändert hat, repariere die Test-Binary, damit der Compiler nicht meckert.

```rust
// In src/bin/stage3_search.rs
use rs_scrape::scraper_roche;
use rs_scrape::web_core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut _browser, page, handle) = web_core::setup_browser().await?;
    // Übergebe 'true' für Debug-Verhalten im isolierten Test
    let urls = scraper_roche::scrape_roche_jobs(&page, true).await?;
    for url in urls {
        println!("{}", url);
    }
    // Cleanup
    _browser.close().await?;
    handle.await?;
    Ok(())
}
```

