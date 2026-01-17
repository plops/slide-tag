import os
import time
import re
from datetime import datetime
from pathlib import Path
from playwright.sync_api import sync_playwright, TimeoutError as PlaywrightTimeoutError
import sqlite_minutils
import loguru

# Configure Logger
log = loguru.logger

# --- CONFIGURATION ---
BASE_URL = "https://www.novartis.com"
# Start at page=0 explicitly
START_URL = "https://www.novartis.com/careers/career-search?search_api_fulltext=&country%5B0%5D=LOC_CH&op=Submit&field_job_posted_date=All&page=0"
SAVE_TO_DB = True
DB_NAME = "novartis_jobs.db"
TIMEOUT_MS = 30000

def get_job_id_fallback(url):
    match = re.search(r'req-([\w-]+)', url, re.IGNORECASE)
    return f"REQ-{match.group(1)}" if match else f"unknown_{int(time.time())}"

def save_to_folder(job_data):
    today = datetime.now().strftime("%Y-%m-%d")
    folder_path = Path(f"novartis_jobs_{today}")
    folder_path.mkdir(exist_ok=True)
    safe_id = re.sub(r'[^a-zA-Z0-9]', '_', job_data['job_id'])
    file_path = folder_path / f"{safe_id}.html"
    with open(file_path, "w", encoding="utf-8") as f:
        f.write(job_data.get('html_content', ''))
    log.info(f"Saved file: {file_path}")

def save_to_database(job_data):
    db = sqlite_minutils.Database(DB_NAME)
    job_data['scraped_at'] = datetime.now().isoformat()
    db["jobs"].upsert(job_data, pk="job_id")
    log.success(f"Saved to DB: {job_data['job_id']}")

def extract_job_details(page, url):
    """Parses details from the specific job page."""
    log.debug(f"Extracting details from {url}")

    # Safely get text helper
    def get_text(selector, default=""):
        try:
            el = page.locator(selector).first
            return el.inner_text().strip() if el.count() > 0 else default
        except:
            return default

    # 1. Job ID
    job_id = get_text(".field_job_id .d-inline-block:last-child", get_job_id_fallback(url))

    # 2. Title
    title = get_text("h1.title", page.title())

    # 3. Description
    description = get_text(".job_description .field--type-text-with-summary")

    # 4. Metadata Helper
    def get_meta_value(class_name):
        return get_text(f".{class_name} .col-6:last-child", None)

    data = {
        "job_id": job_id,
        "title": title,
        "url": url,
        "description": description,
        "division": get_meta_value("field_job_division"),
        "business_unit": get_meta_value("field_job_business_unit"),
        "site": get_meta_value("field_job_work_location"),
        "location": get_meta_value("field_job_country"),
        "job_type": get_meta_value("field_job_type"),
        "posted_date": get_text(".field_job_last_updated .d-inline-block:last-child"),
        "apply_url": page.locator("a.link_button").first.get_attribute("href") if page.locator("a.link_button").count() > 0 else None,
        "html_content": page.content()
    }
    return data

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=False)
        context = browser.new_context()
        page = context.new_page()

        log.info(f"Navigating to {START_URL}")
        page.goto(START_URL)

        # Cookie Banner
        try:
            log.debug("Checking for cookie banner...")
            page.wait_for_selector("#onetrust-accept-btn-handler", state="visible", timeout=5000)
            page.click("#onetrust-accept-btn-handler")
            log.info("Accepted Cookies")
            time.sleep(1) # Allow banner to fade
        except:
            log.debug("No cookie banner found or already accepted.")

        all_job_links = set()
        page_num = 0

        # --- PAGINATION LOOP ---
        while True:
            log.info(f"--- Processing Page {page_num} ---")
            log.debug(f"Current URL: {page.url}")

            # 1. Wait for Table
            try:
                page.wait_for_selector("table.views-table", state="visible", timeout=10000)
            except PlaywrightTimeoutError:
                log.error("Table not found on this page. Stopping pagination.")
                break

            # 2. Extract Links
            log.debug("Looking for job rows...")
            links = page.locator("table.views-table tbody tr .views-field-field-job-title a").all()

            new_count = 0
            for link in links:
                href = link.get_attribute("href")
                if href:
                    full_url = BASE_URL + href if href.startswith("/") else href
                    if full_url not in all_job_links:
                        all_job_links.add(full_url)
                        new_count += 1

            log.info(f"Found {new_count} new jobs on page {page_num}. Total unique: {len(all_job_links)}")

            # 3. Pagination Logic
            log.debug("Checking for 'Next' button...")

            # Novartis specific: "li.pager__item--next" contains the link
            next_li = page.locator("li.pager__item--next")
            next_link = next_li.locator("a")

            if next_li.count() == 0:
                log.info("Pagination: 'Next' list item (li) NOT found. Reached end of results.")
                break

            # Check if disabled
            # The 'a' tag might have class="page-link btn disabled" or the LI might have it
            li_classes = next_li.get_attribute("class") or ""
            a_classes = next_link.get_attribute("class") or ""
            href = next_link.get_attribute("href")

            log.debug(f"Next Button Analysis -> LI Class: '{li_classes}' | A Class: '{a_classes}' | Href: '{href}'")

            if "disabled" in a_classes or not href:
                log.info("Pagination: Next button is present but DISABLED. Reached end of results.")
                break

            # 4. Click and Wait
            try:
                log.info(f"Clicking Next (moving to page {page_num + 1})...")

                # We expect the URL to change to 'page={page_num+1}'
                # We start waiting for the URL *before* we click to avoid race conditions
                with page.expect_navigation(wait_until="domcontentloaded", timeout=TIMEOUT_MS):
                    next_link.click()

                # Check URL updated
                page_num += 1
                log.debug(f"Navigation successful. New URL: {page.url}")

            except PlaywrightTimeoutError:
                log.error("Timed out waiting for next page to load.")
                # Fallback: Check if we are still on the same page
                if f"page={page_num}" not in page.url:
                    log.warning("URL did not update as expected, but continuing to scrape attempt.")
                else:
                    log.error("Stuck on same page. Stopping.")
                    break
            except Exception as e:
                log.error(f"Unexpected error during pagination: {e}")
                break

        log.info(f"Collection complete. Found {len(all_job_links)} jobs. Starting download...")

        # --- DETAIL PAGE LOOP ---
        for index, url in enumerate(all_job_links):
            log.info(f"[{index + 1}/{len(all_job_links)}] Scraping: {url}")
            try:
                page.goto(url)
                # "domcontentloaded" is faster and sufficient for text extraction
                page.wait_for_load_state("domcontentloaded")

                job_data = extract_job_details(page, url)

                if SAVE_TO_DB:
                    save_to_database(job_data)
                else:
                    save_to_folder(job_data)

                # Slight delay to be polite
                time.sleep(0.5)

            except Exception as e:
                log.error(f"Error scraping {url}: {e}")

        browser.close()

if __name__ == "__main__":
    run()