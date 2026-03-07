use anyhow::Result;
use chromiumoxide::Page;
use std::collections::HashSet;
use std::fs;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

/// Navigate to Roche careers search results page
pub async fn navigate_to_roche(page: &Page) -> Result<()> {
    println!("Navigating to Roche careers search results...");
    page.goto("https://careers.roche.com/de/de/search-results")
        .await?;
    page.wait_for_navigation().await?;
    println!(
        "Navigated to: {}",
        page.url().await?.unwrap_or_else(|| "No URL".to_string())
    );
    Ok(())
}

/// Handle cookie banner by accepting or rejecting
pub async fn handle_cookie_banner(page: &Page) -> Result<()> {
    println!("Handling cookie banner...");
    // Wait for cookie banner
    sleep(Duration::from_secs(1)).await;
    // Try to accept
    let accept_js = "document.querySelector('#onetrust-accept-btn-handler')?.click();";
    if page.evaluate(accept_js).await.is_ok() {
        println!("Accepted cookies.");
    } else if page
        .evaluate("document.querySelector('#onetrust-reject-all-handler')?.click();")
        .await
        .is_ok()
    {
        println!("Rejected cookies.");
    } else {
        println!("No cookie banner action taken.");
    }
    // Wait for banner to disappear
    sleep(Duration::from_secs(1)).await;
    Ok(())
}

/// Click the "Ort" accordion to open location filter
pub async fn click_ort_accordion(page: &Page) -> Result<()> {
    println!("Clicking 'Ort' accordion...");
    sleep(Duration::from_secs(1)).await;
    // Scroll into view
    page.evaluate("document.querySelector('#OrtAccordion').scrollIntoView({block: 'center', inline: 'nearest'});").await?;
    page.evaluate("window.scrollBy(0, -80);").await?;
    page.evaluate("document.querySelector('#OrtAccordion').click();")
        .await?;
    println!("'Ort' accordion clicked.");
    Ok(())
}

/// Enter "Schweiz" in the location input
pub async fn enter_schweiz_filter(page: &Page) -> Result<()> {
    println!("Entering 'Schweiz' in location filter...");
    sleep(Duration::from_secs(1)).await;
    page.evaluate("document.querySelector('#facetInput_2').value = 'Schweiz';")
        .await?;
    println!("'Schweiz' entered.");
    Ok(())
}

/// Click the "Schweiz" checkbox
pub async fn click_schweiz_checkbox(page: &Page) -> Result<()> {
    println!("Clicking 'Schweiz' checkbox...");
    let js = r#"
    const sel = "input[data-ph-at-text='Schweiz']";
    const el = document.querySelector(sel);
    let success = false;
    if (el) {
        const target = el.closest('label') || el;
        target.scrollIntoView({block: 'center', inline: 'nearest'});
        window.scrollBy(0, -80);
        const rect = target.getBoundingClientRect();
        if (rect.width !== 0 && rect.height !== 0) {
            try {
                target.click();
                success = true;
            } catch (e) {
                try {
                    target.dispatchEvent(new MouseEvent('click', {bubbles: true, cancelable: true, view: window}));
                    success = true;
                } catch (e2) {
                    // ignore
                }
            }
        }
    }
    success
    "#;
    let result = page.evaluate(js).await?;
    let clicked = result.value().unwrap().as_bool().unwrap_or(false);
    if clicked {
        println!("'Schweiz' checkbox clicked.");
    } else {
        anyhow::bail!("Failed to click 'Schweiz' checkbox");
    }
    Ok(())
}

/// Collect job URLs by paginating through results
pub async fn collect_job_urls(page: &Page) -> Result<Vec<String>> {
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

        // Collect hrefs before clicking next
        let result = page.evaluate(r#"
            Array.from(document.querySelectorAll("a[data-ph-at-id='job-link']")).map(a => a.href).filter(href => href)
        "#).await?;
        let hrefs: Vec<String> = serde_json::from_value(result.value().unwrap().clone())?;
        let prev_count = links.len();
        for href in hrefs {
            links.insert(href.clone());
        }

        // Dump HTML before click
        let html_before = page.content().await?;
        fs::write("page_before_click.html", &html_before)?;
        println!("Dumped HTML to page_before_click.html");

        // Check for next button
        let next_result = page
            .evaluate("!!document.querySelector('a[data-ph-at-id=\"pagination-next-link\"]')")
            .await?;
        let next_exists = next_result.value().unwrap().as_bool().unwrap_or(false);
        if next_exists {
            let href_result = page.evaluate("!!document.querySelector('a[data-ph-at-id=\"pagination-next-link\"]').getAttribute('href')").await?;
            let has_href = href_result.value().unwrap().as_bool().unwrap_or(false);
            println!(
                "Next button exists: {}, has href: {}",
                next_exists, has_href
            );
            if has_href {
                // Get href and navigate
                let href_result = page
                    .evaluate(
                        "document.querySelector('a[data-ph-at-id=\"pagination-next-link\"]').href",
                    )
                    .await?;
                let href: String = serde_json::from_value(href_result.value().unwrap().clone())?;
                println!("Navigating to next page: {}", href);
                page.goto(href).await?;
                page.wait_for_navigation().await?;
                println!(
                    "Navigated to: {}",
                    page.url().await?.unwrap_or_else(|| "No URL".to_string())
                );
                // Dump HTML after wait
                let html_after = page.content().await?;
                fs::write("page_after_click.html", &html_after)?;
                println!("Dumped HTML to page_after_click.html");
            } else {
                println!("Next button disabled, last page.");
                break;
            }
        } else {
            println!("No next button, last page.");
            break;
        }
    }

    let mut sorted_links: Vec<String> = links.into_iter().collect();
    sorted_links.sort();
    println!("Collected {} job links.", sorted_links.len());
    Ok(sorted_links)
}

/// Main scraper function
pub async fn scrape_roche_jobs(page: &Page) -> Result<Vec<String>> {
    navigate_to_roche(page).await?;
    handle_cookie_banner(page).await?;
    click_ort_accordion(page).await?;
    enter_schweiz_filter(page).await?;
    click_schweiz_checkbox(page).await?;
    collect_job_urls(page).await
}
