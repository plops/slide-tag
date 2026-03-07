use rs_scrape::web_core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = web_core::test_browser_title().await?;
    println!("Page title: {}", title);
    Ok(())
}
