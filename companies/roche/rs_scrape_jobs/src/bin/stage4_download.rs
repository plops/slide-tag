use rs_scrape::downloader;
use rs_scrape::json_extractor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls: Vec<String> = std::env::args().skip(1).collect();

    if urls.is_empty() {
        eprintln!(
            "Usage: cargo run --bin stage4_download --features \"scraper\" <url1> <url2> ..."
        );
        std::process::exit(1);
    }

    std::fs::create_dir_all("jobs_html")?;

    let pages = downloader::download_pages(urls).await?;

    for (i, (url, html)) in pages.iter().enumerate() {
        match json_extractor::extract_phapp_json_regex(html) {
            Ok(json) => {
                let filename = format!("jobs_html/job_{}.json", i + 1);
                std::fs::write(&filename, &json)?;
                println!("Saved {} from {}", filename, url);
            }
            Err(e) => {
                let filename = format!("jobs_html/job_{}_failed.html", i + 1);
                std::fs::write(&filename, html)?;
                eprintln!("Failed to extract JSON from {}: {}", url, e);
                eprintln!("Saved HTML to {} for inspection", filename);
            }
        }
    }

    Ok(())
}
