use anyhow::Result;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Download a single page
async fn download_page(client: Arc<Client>, url: String) -> Result<(String, String)> {
    let response = client.get(&url).send().await?;
    let html = response.text().await?;
    Ok((url, html))
}

/// Download multiple pages concurrently
pub async fn download_pages(urls: Vec<String>) -> Result<Vec<(String, String)>> {
    let client = Arc::new(Client::new());
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
