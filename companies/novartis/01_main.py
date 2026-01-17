
import os
import time
import re
from datetime import datetime
from pathlib import Path
from playwright.sync_api import sync_playwright, expect
import sqlite_minutils  # As requested
import loguru

# Configure Logger
log = loguru.logger

# --- CONFIGURATION ---
BASE_URL = "https://www.novartis.com"
START_URL = "https://www.novartis.com/careers/career-search?search_api_fulltext=&country%5B0%5D=LOC_CH&op=Submit&field_job_posted_date=All&page=0"
SAVE_TO_DB = True
DB_NAME = "novartis_jobs.db"

def get_job_id(url):
    """
    Extracts the Job ID from the URL.
    Matches: /careers/career-search/job/details/req-10068139-senior...
    Returns: req-10068139
    """
    # Looks for 'req-' followed by numbers/letters until a dash or end of string
    match = re.search(r'details/(req-[\w]+)', url)
    if match:
        return match.group(1)

    # Fallback for other URL patterns
    match_generic = re.search(r'/job-details/([^/]+)', url)
    return match_generic.group(1) if match_generic else f"unknown_{int(time.time())}"

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
    """Upserts the job data into SQLite using sqlite_minutils."""
    db = sqlite_minutils.Database(DB_NAME)

    # Add timestamp and raw html
    job_data['scraped_at'] = datetime.now().isoformat()
    job_data['html_content'] = html_content

    # Upsert based on job_id (updates if exists, inserts if new)
    db["jobs"].upsert(job_data, pk="job_id")
    log.success(f"Saved to DB: {job_data['job_id']}")

def run():
    with sync_playwright() as p:
        # Launch browser
        browser = p.chromium.launch(headless=False)
        context = browser.new_context()
        page = context.new_page()

        log.info(f"Navigating to {START_URL}")
        page.goto(START_URL)

        # 1. Handle Cookie Banner
        try:
            page.wait_for_selector("#onetrust-accept-btn-handler", state="visible", timeout=10000)
            page.click("#onetrust-accept-btn-handler")
            log.info("Accepted Cookies")
        except:
            log.warning("No cookie banner found or already accepted.")

        all_job_links = set()

        # 2. Pagination Loop: Collect all Job URLs first
        while True:
            # Wait for the table to appear (Updated to match your HTML snippet)
            try:
                page.wait_for_selector("table.views-table", state="visible", timeout=10000)
            except:
                log.error("Table not found. Ending pagination.")
                break

            # Extract links specifically from the table rows
            # We look for the 'a' tag inside the job title cell
            links = page.locator("table.views-table tbody tr .views-field-field-job-title a").all()

            current_page_count = 0
            for link in links:
                href = link.get_attribute("href")
                if href:
                    full_url = BASE_URL + href if href.startswith("/") else href
                    if full_url not in all_job_links:
                        all_job_links.add(full_url)
                        current_page_count += 1

            log.info(f"Found {current_page_count} new jobs on this page. Total unique: {len(all_job_links)}")

            # Check for "Next" button
            # Selector matches the Drupal pager structure from your snippet
            next_button = page.locator("li.pager__item--next a")

            if next_button.count() > 0 and next_button.is_visible():
                next_button.click()
                page.wait_for_load_state("networkidle")
                time.sleep(1.5) # Polite delay
            else:
                log.info("No more pages found (Next button missing or hidden).")
                break

        log.info(f"Collection complete. Starting download of {len(all_job_links)} jobs...")

        # 3. Visit each job and extract details
        for index, url in enumerate(all_job_links):
            log.info(f"[{index + 1}/{len(all_job_links)}] Scraping: {url}")

            try:
                page.goto(url)
                page.wait_for_load_state("domcontentloaded")

                # Extraction
                job_id = get_job_id(url)
                title = page.title()
                content_html = page.content()

                # Cleanup title if H1 exists
                h1 = page.locator("h1")
                if h1.count() > 0:
                    title = h1.first.inner_text().strip()

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

                time.sleep(1)

            except Exception as e:
                log.error(f"Error scraping {url}: {e}")

        browser.close()

if __name__ == "__main__":
    run()
