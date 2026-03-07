use anyhow::Result;
use chromiumoxide::{Browser, BrowserConfig, Page};
use futures::StreamExt;

pub async fn test_browser_title() -> Result<String> {
    println!("Building browser config...");
    let config = BrowserConfig::builder()
        .build()
        .map_err(anyhow::Error::msg)?;
    println!("Launching browser...");
    let (mut browser, mut handler) = Browser::launch(config).await?;
    println!("Browser launched successfully.");

    let handle = tokio::spawn(async move {
        println!("Starting handler loop...");
        while let Some(h) = handler.next().await {
            if h.is_err() {
                println!("Handler error: {:?}", h);
            }
        }
        println!("Handler loop ended.");
    });

    println!("Creating new page...");
    let page = browser.new_page("https://www.google.com").await?;
    println!("Page created, waiting for load...");

    println!("Getting page title...");
    let title = page
        .get_title()
        .await?
        .unwrap_or_else(|| "No title".to_string());
    println!("Title retrieved: {}", title);

    println!("Closing browser...");
    browser.close().await?;
    println!("Browser closed, waiting for handler...");
    handle.await?;
    println!("Handler finished.");

    Ok(title)
}

/// Setup browser with headless Chrome and return a page
pub async fn setup_browser() -> Result<(Browser, Page, tokio::task::JoinHandle<()>)> {
    println!("Building browser config for scraper...");
    let config = BrowserConfig::builder()
        .arg("--headless=new")
        .arg("--window-size=1920,1080")
        .arg("--no-sandbox")
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
