# Chromiumoxide Documentation & Examples

`chromiumoxide` provides a high-level, asynchronous API for controlling Chrome/Chromium via the DevTools Protocol.

## Core Features
- **Async/Await**: Built for `tokio` or `async-std`.
- **Headless Support**: Can run in the background.
- **Full Control**: Navigate, click, type, and extract data.

## Examples

### Setup and Navigation
```rust
use futures::StreamExt;
use chromiumoxide::{Browser, BrowserConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder().with_head().build()?
    ).await?;

    // The handler MUST be polled in a separate task
    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() { break; }
        }
    });

    let page = browser.new_page("https://www.google.com").await?;
    
    // Wait for the page to load
    page.goto("https://www.google.com").await?;
    
    // Interaction
    page.find_element("input[name='q']")
        .await?
        .type_str("Rust programming")
        .await?
        .press_key("Enter")
        .await?;

    browser.close().await?;
    handle.await?;
    Ok(())
}
```

### Extraction with XPath/Selectors
```rust
// Find multiple elements
let links = page.find_elements("a.job-link").await?;
for link in links {
    let href = link.attribute("href").await?;
    println!("Found job link: {:?}", href);
}

// Using XPath
let element = page.find_xpath("//div[@class='job-detail']").await?;
let text = element.inner_text().await?;
```

## Relevant Tasks in This Project
- Scraping job links from the Roche careers site (Phase 1).
- Handling dynamic content or pagination using browser automation.
- Extracting full job descriptions from detail pages.
