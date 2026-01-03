from playwright.sync_api import sync_playwright
import re

# Start Playwright
pw = sync_playwright().start()

# Launch Chromium with DevTools and visible UI
browser = pw.chromium.launch(headless=False, devtools=True)
context = browser.new_context()
page = context.new_page()

# Navigate to the target site
page.goto("https://careers.roche.com/de/de/search-results")
# Accept cookies
page.click("#onetrust-accept-btn-handler")
page.click("#OrtAccordion")
page.fill("#facetInput_2", "Schweiz")
page.get_by_role('checkbox', name=re.compile(r'^Schweiz')).check()
page.select_option("#sortselect", "Neuesten")