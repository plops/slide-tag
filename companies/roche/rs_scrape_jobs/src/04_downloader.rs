use anyhow::Result;
use reqwest::{header, Client};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Build a reqwest client configured to mimic Google Chrome
fn build_browser_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();

    // 1. Pretend to be Windows Chrome
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"),
    );

    // 2. Tell CloudFront we accept standard HTML
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"),
    );

    // 3. Match typical browser language settings
    headers.insert(
        header::ACCEPT_LANGUAGE,
        header::HeaderValue::from_static("en-US,en;q=0.9,de;q=0.8"),
    );

    // 4. Sec-CH and Sec-Fetch headers that modern Chrome always sends
    headers.insert(
        "sec-ch-ua",
        header::HeaderValue::from_static(
            "\"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", header::HeaderValue::from_static("?0"));
    headers.insert(
        "sec-ch-ua-platform",
        header::HeaderValue::from_static("\"Windows\""),
    );
    headers.insert(
        "sec-fetch-dest",
        header::HeaderValue::from_static("document"),
    );
    headers.insert(
        "sec-fetch-mode",
        header::HeaderValue::from_static("navigate"),
    );
    headers.insert("sec-fetch-site", header::HeaderValue::from_static("none"));
    headers.insert("sec-fetch-user", header::HeaderValue::from_static("?1"));
    headers.insert(
        "upgrade-insecure-requests",
        header::HeaderValue::from_static("1"),
    );

    // Build the client with default headers and browser compression support
    let client = Client::builder()
        .default_headers(headers)
        .brotli(true)
        .gzip(true)
        .build()?;

    Ok(client)
}

/// Download a single page
async fn download_page(client: Arc<Client>, url: String) -> Result<(String, String)> {
    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    Ok((url, html))
}

/// Download multiple pages concurrently
pub async fn download_pages(urls: Vec<String>) -> Result<Vec<(String, String)>> {
    // USE THE DISGUISED CLIENT HERE instead of Client::new()
    let client = Arc::new(build_browser_client()?);
    let (tx, mut rx) = mpsc::channel(100);

    // Spawn download tasks
    for url in urls {
        let client = Arc::clone(&client);
        let tx = tx.clone();
        tokio::spawn(async move {
            match download_page(client, url.clone()).await {
                Ok(result) => {
                    let _ = tx.send(Ok(result)).await;
                }
                Err(e) => {
                    eprintln!("Failed to download {}: {}", url, e);
                    let _ = tx.send(Err(e)).await;
                }
            }
        });
    }

    // Drop the original sender so the receiver knows when all tasks are done
    drop(tx);

    let mut results = Vec::new();
    while let Some(result) = rx.recv().await {
        if let Ok(page) = result {
            results.push(page);
        }
    }

    Ok(results)
}
