# # 1. Create/Activate virtual environment
# uv venv
# source .venv/bin/activate  # or .venv/bin/activate.fish / .csh
#
# # 2. Install Python dependencies
# uv pip install playwright loguru
#
# # 3. Install the browser binary (Playwright manages this internally)
# uv run playwright install chromium

from playwright.sync_api import sync_playwright, TimeoutError as PlaywrightTimeoutError
from loguru import logger
import time

# Global variables for REPL debugging
p = None
browser = None
context = None
page = None
all_job_links = set()

# ============================================================================
# SETUP BROWSER
# ============================================================================
logger.info("Setting up browser...")
p = sync_playwright().start()
browser = p.chromium.launch(
    headless=False,
    args=["--start-maximized"]
)
context = browser.new_context(viewport={"width": 1920, "height": 1080})
page = context.new_page()
logger.info("Browser ready.")

# ============================================================================
# NAVIGATE TO CAREERS PAGE
# ============================================================================
url = "https://careers.roche.com/de/de/search-results"
logger.info(f"Navigating to {url}")
page.goto(url)
time.sleep(0.5)

# ============================================================================
# HANDLE COOKIE BANNER
# ============================================================================
try:
    logger.info("Checking for cookie banner...")
    accept_btn = page.locator("#onetrust-accept-btn-handler")
    accept_btn.wait_for(state="visible", timeout=5000)
    logger.info("Cookie banner found. Clicking accept...")
    accept_btn.click()
    page.locator("#onetrust-button-group-parent").wait_for(state="hidden", timeout=5000)
    logger.info("Cookie banner accepted and dismissed.")
except PlaywrightTimeoutError:
    logger.warning("Cookie banner did not appear or could not be clicked. Continuing...")

# ============================================================================
# CLICK ORT ACCORDION
# ============================================================================
logger.info("Opening 'Ort' accordion...")
page.locator("#OrtAccordion").click()
time.sleep(0.3)

# ============================================================================
# FILTER SCHWEIZ
# ============================================================================
logger.info("Entering 'Schweiz' into filter...")
input_locator = page.locator("#facetInput_2")
input_locator.fill("Schweiz")
time.sleep(0.3)

# ============================================================================
# SELECT SCHWEIZ CHECKBOX
# ============================================================================
logger.info("Selecting 'Schweiz' checkbox...")
checkbox_label = page.locator("label").filter(has=page.locator("input[data-ph-at-text='Schweiz']"))
checkbox_label.click()
time.sleep(1)
# Wait for job list to render instead of waiting for networkidle
try:
    page.locator("a[data-ph-at-id='job-link']").first.wait_for(timeout=10000)
    logger.info("Schweiz selected. Job list loaded.")
except PlaywrightTimeoutError:
    logger.warning("Job list did not load after selecting Schweiz. Continuing anyway...")

# ============================================================================
# COLLECT JOBS - PAGINATE AND EXTRACT LINKS
# ============================================================================
logger.info("Starting pagination and job collection...")
while True:
    # Wait for job links to appear
    try:
        page.locator("a[data-ph-at-id='job-link']").first.wait_for(timeout=10000)
    except PlaywrightTimeoutError:
        logger.warning("No job links found on this page.")
        break

    # Extract all hrefs on current page
    current_links = page.eval_on_selector_all(
        "a[data-ph-at-id='job-link']",
        "elements => elements.map(e => e.href)"
    )
    all_job_links.update(current_links)
    logger.info(f"Found {len(current_links)} links on page. Total unique: {len(all_job_links)}")

    # Check for Next Button
    next_btn = page.locator("a[data-ph-at-id='pagination-next-link']")

    if next_btn.is_visible():
        href = next_btn.get_attribute("href")
        if href and "javascript" not in href:
            try:
                next_btn.click()
                time.sleep(1)
                # Wait for job list to render on next page
                page.locator("a[data-ph-at-id='job-link']").first.wait_for(timeout=10000)
            except PlaywrightTimeoutError:
                logger.warning("Timeout waiting for next page to load, or last page reached.")
                break
        else:
            logger.info("Next button exists but has no valid href. Last page reached.")
            break
    else:
        logger.info("No 'Next' button visible. Last page reached.")
        break

# ============================================================================
# SAVE RESULTS
# ============================================================================
logger.info(f"Finished. Collected {len(all_job_links)} total jobs.")
with open("jobs.txt", "w") as f:
    for link in sorted(all_job_links):
        f.write(link + "\n")
logger.info("Saved to jobs.txt")

# ============================================================================
# CLEANUP
# ============================================================================
browser.close()
p.stop()
logger.info("Browser closed.")
