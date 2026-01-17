import os
import time
import re
from datetime import datetime
from pathlib import Path
from playwright.sync_api import sync_playwright, expect
import sqlite_minutils
import loguru

log = loguru.logger

# --- CONFIGURATION ---
BASE_URL = "https://www.novartis.com"
START_URL = "https://www.novartis.com/careers/career-search?search_api_fulltext=&country%5B0%5D=LOC_CH&op=Submit&field_job_posted_date=All&page=0"
SAVE_TO_DB = True  # Set to False to save to folders instead
DB_NAME = "novartis_jobs.db"

def get_job_id(url):
    """Extracts the Job ID (e.g., 390291BR) from the URL."""
    match = re.search(r'/job-details/([^/]+)', url)
    return match.group(1) if match else "unknown_id"

def save_to_folder(job_data, html_content):
    """Saves the raw HTML to a dated folder."""
    today = datetime.now().strftime("%Y-%m-%d")
    folder_path = Path(f"novartis_jobs_{today}")
    folder_path.mkdir(exist_ok=True)

    # Sanitize filename
    safe_id = re.sub(r'[^a-zA-Z0-9]', '_', job_data['job_id'])
    file_path = folder_path / f"{safe_id}.html"

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(html_content)
    log.info(f"Saved file: {file_path}")

def save_to_database(job_data, html_content):
    """Upserts the job data into SQLite."""
    db = sqlite_minutils.Database(DB_NAME)

    # Add timestamp and raw html
    job_data['scraped_at'] = datetime.now().isoformat()
    job_data['html_content'] = html_content

    # Upsert based on job_id (updates if exists, inserts if new)
    db["jobs"].upsert(job_data, pk="job_id")
    log.info(f"Saved to DB: {job_data['job_id']}")

def run():
    with sync_playwright() as p:
        # Launch browser (headless=False allows you to see what's happening)
        browser = p.chromium.launch(headless=False)
        context = browser.new_context()
        page = context.new_page()

        log.info(f"Navigating to {START_URL}")
        page.goto(START_URL)

        # 1. Handle Cookie Banner (Crucial for Novartis site)
        try:
            # Wait a moment for the banner to appear
            page.wait_for_selector("#onetrust-accept-btn-handler", timeout=15000)
            page.click("#onetrust-accept-btn-handler")
            log.info("Accepted Cookies")
        except:
            log.info("No cookie banner found or already accepted.")

        all_job_links = set()

        # 2. Pagination Loop: Collect all Job URLs first
        while True:
            # Wait for job cards to load
            page.wait_for_selector(".views-row", state="attached")

            # Extract links on current page
            # Novartis usually puts links inside h4 or specific class inside the card
            # Adjust selector based on current site structure
            links = page.locator("a[href*='/job-details/']").all()

            current_page_count = 0
            for link in links:
                href = link.get_attribute("href")
                if href:
                    full_url = BASE_URL + href if href.startswith("/") else href
                    if full_url not in all_job_links:
                        all_job_links.add(full_url)
                        current_page_count += 1

            log.info(f"Found {current_page_count} jobs on this page. Total unique: {len(all_job_links)}")

            # Check for "Next" button
            # Novartis uses a specific pager class. We look for the 'next' icon or text.
            next_button = page.locator(".pager__item--next a")

            if next_button.count() > 0 and next_button.is_visible():
                next_button.click()
                # Wait for the network to idle to ensure new content loaded
                page.wait_for_load_state("networkidle")
                time.sleep(2) # Polite delay
            else:
                log.info("No more pages found.")
                break

        log.info(f"collection complete. Starting download of {len(all_job_links)} jobs...")

        # 3. Visit each job and extract details
        for index, url in enumerate(all_job_links):
            log.info(f"[{index + 1}/{len(all_job_links)}] Scraping: {url}")

            try:
                page.goto(url)
                page.wait_for_load_state("domcontentloaded")

                # Basic Extraction
                job_id = get_job_id(url)
                title = page.title()
                content_html = page.content()

                # Try to get H1 for cleaner title
                h1_locator = page.locator("h1")
                if h1_locator.count() > 0:
                    title = h1_locator.first.inner_text().strip()

                job_data = {
                    "job_id": job_id,
                    "title": title,
                    "url": url,
                }

                # 4. Storage Selection
                if SAVE_TO_DB:
                    save_to_database(job_data, content_html)
                else:
                    save_to_folder(job_data, content_html)

                # Be polite to the server
                time.sleep(1)

            except Exception as e:
                log.info(f"Error scraping {url}: {e}")

        browser.close()

if __name__ == "__main__":
    run()