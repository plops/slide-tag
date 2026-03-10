scraping on the server is not working. 
root@ubuntu-2gb-ash-1:/var/www/jobs.rocketrecap.com# journalctl -f
Mar 10 22:21:05 ubuntu-2gb-ash-1 rs-scrape[158254]: DEBUG root: Found user - ID: Some(1), Name: plops
Mar 10 22:21:21 ubuntu-2gb-ash-1 rs-scrape[158254]: 2026-03-10T22:21:21.319388Z  INFO rs_scrape::admin: Admin triggered manual scrape. Debug: false
Mar 10 22:21:21 ubuntu-2gb-ash-1 rs-scrape[158254]: Building browser config for scraper...
Mar 10 22:21:21 ubuntu-2gb-ash-1 rs-scrape[158254]: Launching browser...
Mar 10 22:21:21 ubuntu-2gb-ash-1 kernel: traps: chrome[158260] trap int3 ip:5fa54a78792c sp:7ffed4c930f0 error:0 in chrome[5fa547aab000+c75e000]
Mar 10 22:21:21 ubuntu-2gb-ash-1 rs-scrape[158254]: 2026-03-10T22:21:21.808132Z ERROR rs_scrape::admin: Manual scrape failed: Browser process exited with status ExitStatus(unix_wait_status(133)) before websocket URL could be resolved, stderr: BrowserStderr("chrome_crashpad_handler: --database is required\nTry 'chrome_crashpad_handler --help' for more information.\n[158260:158260:0310/222121.548894:ERROR:third_party/crashpad/crashpad/util/linux/socket.cc:120] recvmsg: Connection reset by peer (104)\n")




Dieser Fehler (`trap int3` im Kernel und der `chrome_crashpad_handler` Fehler) ist ein klassisches Problem, wenn Headless Chrome auf einem Linux-Server (insbesondere unter Ubuntu) als systemd-Service gestartet wird. 

Chrome stürzt in den ersten Millisekunden beim Booten ab. Dafür gibt es auf Ubuntu-Servern typischerweise zwei Hauptgründe:

1. **Snap-Paket Restriktionen:** Wenn Chromium über Ubuntu's Standard-Paketmanager (`apt install chromium-browser`) installiert wurde, ist es in Wirklichkeit ein **Snap-Paket**. Snaps laufen in einer strikten AppArmor-Sandbox und benötigen zwingend ein `/home/`-Verzeichnis des ausführenden Users. Dein systemd-Service läuft aber vermutlich als `www-data` (ohne echtes Home-Verzeichnis). Das führt zum sofortigen Absturz.
2. **Fehlendes Shared Memory & GPU-Konflikte:** Server-Umgebungen und systemd-Services haben oft ein stark limitiertes `/dev/shm` (Shared Memory), was Chrome zum Absturz bringt.

Hier ist die Lösung in **zwei Schritten**:

### Schritt 1: Browser-Konfiguration in Rust anpassen

Wir müssen Chrome anweisen, kein Shared Memory (`/dev/shm`) zu nutzen, die GPU komplett zu deaktivieren und den Crash-Reporter abzustellen.

Passe in der Datei `src/02_web_core.rs` die `setup_browser` (und idealerweise auch `test_browser_title`) Funktion an:

```rust
// src/02_web_core.rs
use anyhow::Result;
use chromiumoxide::{Browser, BrowserConfig, Page};
use futures::StreamExt;

// ... (test_browser_title ebenfalls anpassen, falls genutzt)

pub async fn setup_browser() -> Result<(Browser, Page, tokio::task::JoinHandle<()>)> {
    println!("Building browser config for scraper...");
    
    let config = BrowserConfig::builder()
        .arg("--headless=new")
        .arg("--window-size=1920,1080")
        .arg("--no-sandbox")
        // NEU: Wichtige Flags für Linux Server / Systemd
        .arg("--disable-dev-shm-usage")      // Nutzt /tmp statt /dev/shm
        .arg("--disable-gpu")                // GPU deaktivieren (hilft bei fehlenden Grafiktreibern)
        .arg("--disable-software-rasterizer")
        .arg("--disable-crash-reporter")     // Verhindert den Crashpad-Fehler im Log
        .arg("--disable-extensions")
        .build()
        .map_err(anyhow::Error::msg)?;

    println!("Launching browser...");
    let (browser, mut handler) = Browser::launch(config).await?;
    println!("Browser launched successfully.");

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                eprintln!("Handler error: {:?}", h);
            }
        }
    });

    let page = browser.new_page("about:blank").await?;
    Ok((browser, page, handle))
}
```

*Kompiliere und deploye die App danach neu.*

---

### Schritt 2: Offizielles Google Chrome (.deb) auf dem Server installieren

Um das Ubuntu-Snap-Problem mit dem `www-data` User endgültig zu umgehen, solltest du auf dem Server das offizielle `.deb` Paket von Google Chrome installieren. `chromiumoxide` wird dieses automatisch finden und bevorzugen.

Führe diese Befehle als `root` (oder via `sudo`) auf deinem Server `ubuntu-2gb-ash-1` aus:

```bash
# 1. Lade das offizielle Chrome Debian-Paket herunter
cd /tmp
wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb

# 2. Installiere es (apt kümmert sich automatisch um fehlende Abhängigkeiten wie libnss3, libgbm1 etc.)
sudo apt-get install -y ./google-chrome-stable_current_amd64.deb

# 3. Optional: Lösche das fehleranfällige Snap-Chromium, falls installiert
sudo snap remove chromium
sudo apt-get remove -y chromium-browser

# 4. Service neu starten
systemctl restart rs-scrape
```

**Warum das hilft:** Das offizielle `.deb` Paket ist kein Snap. Es installiert alle benötigten Headless-Bibliotheken (Schriftarten, X11/Wayland-Dummys, NSS-Zertifikatsbibliotheken) direkt ins System und lässt sich problemlos vom eingeschränkten `www-data` User ausführen. Zusammen mit `--disable-dev-shm-usage` wird das Scraping nun stabil funktionieren.