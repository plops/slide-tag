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

def run():
    logger.info("Starting Playwright scraper...")

    with sync_playwright() as p:
        # Launch options.
        # On Gentoo, if you strictly want to use system chrome, add: executable_path="/usr/bin/google-chrome-stable"
        browser = p.chromium.launch(
            headless=False,
            args=["--start-maximized"] # mimic full screen
        )
        context = browser.new_context(viewport={"width": 1920, "height": 1080})
        page = context.new_page()

        # 1. Navigate
        url = "https://careers.roche.com/de/de/search-results"
        logger.info(f"Navigating to {url}")
        page.goto(url)

        # 2. Cookie Banner (Wait and Click)
        # We look for the accept button. Playwright waits automatically for it to be actionable.
        try:
            logger.info("Checking for cookie banner...")
            # Locator for the 'Accept All' button
            accept_btn = page.locator("#onetrust-accept-btn-handler")

            # Wait up to 5 seconds for it to appear
            accept_btn.click(timeout=5000)

            # Wait for banner to disappear
            page.locator("#onetrust-button-group-parent").wait_for(state="hidden", timeout=5000)
            logger.info("Cookie banner accepted and dismissed.")
        except PlaywrightTimeoutError:
            logger.warning("Cookie banner did not appear or could not be clicked. Continuing...")

        # 3. Click "Ort" Accordion
        # Playwright automatically scrolls elements into view before clicking.
        logger.info("Opening 'Ort' accordion...")
        try:
            page.locator("#OrtAccordion").click()
        except Exception as e:
            logger.error(f"Failed to click Ort Accordion: {e}")
            return

        # 4. Enter "Schweiz" in Filter Input
        logger.info("Entering 'Schweiz' into filter...")
        try:
            # We wait for the animation of the accordion if necessary by waiting for the input to be visible
            input_locator = page.locator("#facetInput_2")
            input_locator.fill("Schweiz")
        except Exception as e:
            logger.error(f"Failed to filter input: {e}")
            return

        # 5. Click "Schweiz" Checkbox
        # Logic: Find the Input with the specific data attribute, but click its parent/associated Label.
        logger.info("Selecting 'Schweiz' checkbox...")
        try:
            # Selector explanation: Find a label that *contains* the specific input inside it.
            # This is much more robust than JS injection.
            checkbox_label = page.locator("label").filter(has=page.locator("input[data-ph-at-text='Schweiz']"))
            checkbox_label.click()

            # Wait a moment for the list to refresh/loading spinner to finish
            # Ideally, wait for the network to settle or a loading overlay to disappear
            page.wait_for_load_state("networkidle")
        except Exception as e:
            logger.error(f"Failed to click country checkbox: {e}")
            return

        # 6. Pagination and Collection
        logger.info("Starting pagination and job collection...")

        all_job_links = set()

        while True:
            # Wait for at least one job link to be present
            try:
                page.locator("a[data-ph-at-id='job-link']").first.wait_for(timeout=10000)
            except PlaywrightTimeoutError:
                logger.warning("No job links found on this page.")
                break

            # Extract all hrefs on current page
            # eval_on_selector_all runs JS in the browser to map elements to hrefs (very fast)
            current_links = page.eval_on_selector_all(
                "a[data-ph-at-id='job-link']",
                "elements => elements.map(e => e.href)"
            )

            initial_count = len(all_job_links)
            all_job_links.update(current_links)
            logger.info(f"Found {len(current_links)} links on page. Total unique: {len(all_job_links)}")

            # Check for Next Button
            next_btn = page.locator("a[data-ph-at-id='pagination-next-link']")

            # Logic: If visible AND has an href attribute (not disabled)
            if next_btn.is_visible():
                href = next_btn.get_attribute("href")
                if href and "javascript" not in href: # Ensure it's a valid link
                    try:
                        # Click and wait for the page to update.
                        # We wait for the URL to change or network idle
                        with page.expect_navigation(wait_until="domcontentloaded", timeout=10000):
                            next_btn.click()
                    except PlaywrightTimeoutError:
                        logger.warning("Timeout waiting for next page navigation, or last page reached.")
                        break
                else:
                    logger.info("Next button exists but has no valid href. Last page reached.")
                    break
            else:
                logger.info("No 'Next' button visible. Last page reached.")
                break

        # 7. Output
        logger.info(f"Finished. Collected {len(all_job_links)} total jobs.")
        with open("jobs.txt", "w") as f:
            for link in sorted(all_job_links):
                f.write(link + "\n")

        logger.info("Saved to jobs.txt")
        browser.close()

if __name__ == "__main__":
    run()