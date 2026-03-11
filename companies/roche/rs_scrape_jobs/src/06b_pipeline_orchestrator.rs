use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::sync::Arc;

#[cfg(feature = "ai")]
use crate::ai_core::AiProvider;

use crate::data_ingestion;
use crate::db_traits::DatabaseProvider;
use crate::downloader;
use crate::json_extractor;
use crate::scraper_roche;
use crate::web_core;

pub async fn run_pipeline(
    repo: Arc<dyn DatabaseProvider>, 
    #[cfg(feature = "ai")] ai: Option<Arc<dyn AiProvider>>,
    #[cfg(not(feature = "ai"))] ai: Option<()>,
    debug_dump: bool
) -> Result<()> {
    // Setup browser
    let (mut browser, page, handle) = web_core::setup_browser().await?;

    // 1. Scrape URLs (Jetzt mit debug_dump Parameter!)
    let urls = scraper_roche::scrape_roche_jobs(&page, debug_dump).await?;

    // Cleanup browser
    browser.close().await?;
    handle.await?;

    // 2. Download pages with politeness
    let pages = downloader::download_pages(urls).await?;

    // 3. Process each page in-memory
    for (url, html) in pages {
        // Optional debug dump
        if debug_dump {
            let date = Utc::now().format("%Y-%m-%d");
            let dir = format!("debug_dumps/{}", date);
            fs::create_dir_all(&dir)?;
            // Save HTML
            let html_path = format!(
                "{}/html_{}.html",
                dir,
                url.replace("/", "_").replace(":", "").replace("?", "_")
            );
            fs::write(&html_path, &html)?;
        }

        // Extract JSON
        let json = match json_extractor::extract_phapp_json_regex(&html) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to extract JSON from {}: {}", url, e);
                continue;
            }
        };

        if debug_dump {
            let date = Utc::now().format("%Y-%m-%d");
            let dir = format!("debug_dumps/{}", date);
            let json_path = format!(
                "{}/json_{}.json",
                dir,
                url.replace("/", "_").replace(":", "").replace("?", "_")
            );
            fs::write(&json_path, &json)?;
        }

        // Parse job
        let job = match data_ingestion::parse_roche_job(&json) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to parse job from {}: {}", url, e);
                continue;
            }
        };

        // Insert into job history
        repo.insert_job_history(&job).await?;
        println!("Inserted job: {}", job.title);
    }

    // AI annotation step at the end of pipeline
    #[cfg(feature = "ai")]
    if let Some(ai_provider) = ai {
        tracing::info!("Scraping abgeschlossen. Starte AI-Annotation für unannotierte Jobs...");
        match crate::ai_workflow::annotate_unannotated_jobs(repo.clone(), ai_provider, 50).await {
            Ok(count) => tracing::info!("Pipeline AI-Schritt: Erfolgreich {} Jobs annotiert.", count),
            Err(e) => tracing::error!("Pipeline AI-Schritt fehlgeschlagen: {:?}", e),
        }
    }

    Ok(())
}
