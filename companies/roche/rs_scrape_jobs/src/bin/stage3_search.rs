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
