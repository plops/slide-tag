from helium import *
import shutil
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from loguru import logger
import time


# configure this browser binary: /usr/bin/google-chrome-stable
chromedriver_path = shutil.which("chromedriver") or "/usr/bin/chromedriver"
chrome_binary = shutil.which("google-chrome-stable") or "/usr/bin/google-chrome-stable"
options = Options()
options.binary_location = chrome_binary
service = Service(chromedriver_path)
driver = webdriver.Chrome(service=service, options=options)
set_driver(driver)

logger.info(f"Using chromedriver at {chromedriver_path}")
logger.info(f"Using chrome binary at {chrome_binary}")
logger.info("Chrome driver started and bound to Helium")

# navigate
logger.info("Navigating to careers.roche.com search results")
go_to("https://careers.roche.com/de/de/search-results")
logger.info(f"Current URL after navigation: {driver.current_url}")

wait = WebDriverWait(driver, 10)

# scroll to the bottom of the page to load all elements

# click on "Ort" //*[@id="OrtAccordion"]
# ensure 'OrtAccordion' is visible (scroll into view) then click
logger.info("Ensuring 'Ort' accordion is visible and clickable")
try:
    ort_accordion = wait.until(EC.presence_of_element_located((By.ID, "OrtAccordion")))

    # scroll element into view and adjust for any sticky header (small negative offset)
    driver.execute_script(
        "arguments[0].scrollIntoView({block: 'center', inline: 'nearest'});"
        "window.scrollBy(0, -80);", ort_accordion
    )

    # optional short wait for any animations / reflow
    wait.until(EC.element_to_be_clickable((By.ID, "OrtAccordion")))

    # click (use JS click if normal click is still blocked)
    try:
        ort_accordion.click()
    except Exception:
        driver.execute_script("arguments[0].click();", ort_accordion)

    logger.info("'Ort' accordion clicked")
except Exception:
    logger.exception("Failed to click 'Ort' accordion")
    driver.quit()
    exit(1)


# click away the cookie banner (note that it appears with a slight delay):
# <div id="onetrust-button-group-parent" class="ot-sdk-three ot-sdk-columns has-reject-all-button"><div id="onetrust-button-group"><button id="onetrust-pc-btn-handler">Cookie-Einstellungen</button> <button id="onetrust-reject-all-handler">Alle ablehnen</button> <button id="onetrust-accept-btn-handler">Alle akzeptieren</button></div></div>
# xpath to accept all: //*[@id="onetrust-accept-btn-handler"]
logger.info("Waiting for cookie banner and attempting to accept")

try:
    accept = WebDriverWait(driver, 10).until(
        EC.element_to_be_clickable((By.ID, "onetrust-accept-btn-handler"))
    )
    logger.info("Cookie accept button found, clicking accept")
    driver.execute_script("arguments[0].click();", accept)
    # wait for the banner to disappear
    WebDriverWait(driver, 5).until(
        EC.invisibility_of_element_located((By.ID, "onetrust-button-group-parent"))
    )
    logger.info("Cookie banner disappeared after accepting")
except Exception:
    logger.exception("Failed to click accept cookie button, trying reject or ignoring")
    # fallback: try the "reject all" button or ignore if none present
    try:
        reject = driver.find_element(By.ID, "onetrust-reject-all-handler")
        logger.info("Cookie reject button found, clicking reject")
        driver.execute_script("arguments[0].click();", reject)
        WebDriverWait(driver, 5).until(
            EC.invisibility_of_element_located((By.ID, "onetrust-button-group-parent"))
        )
        logger.info("Cookie banner disappeared after rejecting")
    except Exception:
        logger.exception("Could not interact with cookie banner; continuing without dismissing it")


# Enter "Schweiz" in the "Ort" filter input box with xpath //*[@id="facetInput_2"]
logger.info("Entering 'Schweiz' in the 'Ort' filter input box")
try:
    ort_input = wait.until(EC.presence_of_element_located((By.ID, "facetInput_2")))
    ort_input.clear()
    ort_input.send_keys("Schweiz")
    logger.info("'Schweiz' entered in the 'Ort' filter input box")
except Exception:
    logger.exception("Failed to enter 'Schweiz' in the 'Ort' filter input box")
    driver.quit()
    exit(1)

# Click on "Schweiz" checkbox with xpath //*[@id="country_phs_0"] or even better by css selector input[data-ph-at-text='Schweiz']
# As before it is required to scroll the element into view
# we have to click on the nearest label, not the input itself

try:
    logger.info("Locating 'Schweiz' checkbox via CSS only")
    selector = "input[data-ph-at-text='Schweiz']"
    # ensure the element exists in the DOM first
    wait.until(EC.presence_of_element_located((By.CSS_SELECTOR, selector)))

    logger.info("'Schweiz' checkbox present, attempting JS scroll+click on nearest label")
    js_click_and_scroll = """
    const sel = arguments[0];
    const el = document.querySelector(sel);
    if (!el) return false;
    const target = el.closest('label') || el;
    target.scrollIntoView({block: 'center', inline: 'nearest'});
    window.scrollBy(0, -80); // offset for sticky header
    const rect = target.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return false;
    try {
        target.click();
        return true;
    } catch (e) {
        // fallback dispatch
        try {
            target.dispatchEvent(new MouseEvent('click', {bubbles: true, cancelable: true, view: window}));
            return true;
        } catch (e2) {
            return false;
        }
    }
    """

    clicked = WebDriverWait(driver, 10, poll_frequency=0.5).until(
        lambda d: d.execute_script(js_click_and_scroll, selector)
    )

    if clicked:
        logger.info("'Schweiz' checkbox clicked (CSS + JS on label)")
    else:
        raise Exception("JS click returned false")
except Exception:
    logger.exception("Failed to click 'Schweiz' checkbox using CSS + JS")
    driver.quit()
    exit(1)


# There will be a list of jobs. I think this is a good selector for the ul element: ph-role="data.bind:jobResults"
# I only want the corresponding link, e.g. https://careers.roche.com/de/de/job/202203-110282/System-Development-Troubleshooter
# <ul data-ph-at-id="jobs-list" ph-role="data.bind:jobResults" data-ps="af1c0deb-ul-2" v-phw-setting="" class="au-target" au-target-id="103" data-ph-at-widget-data-count="10" role="list">
#                     <li class="jobs-list-item au-target phw-card-block-nd" data-ph-at-id="jobs-list-item" data-ps="af1c0deb-li-3" v-phw-setting="" au-target-id="104" role="listitem">
#                         <!--anchor-->
# <div data-ps="af1c0deb-div-27" v-phw-setting="" class="au-target" au-target-id="111">
#     <div class="information au-target" data-ps="af1c0deb-div-28" v-phw-setting="" au-target-id="112">
#
#         <!--anchor-->
#
#         <div class="job-smart-tags au-target" data-ps="af1c0deb-div-31" v-phw-setting="" au-target-id="119">
#             <!--anchor-->
#             <div data-ps="af1c0deb-div-33" v-phw-setting="" class="au-target job-tag-area style-1" au-target-id="127">
#                 <!--anchor-->
#                 <!--anchor-->
#             </div>
#         </div>
#         <span role="heading" key-role="headingRole" aria-level="3" key-aria-level="headingAriaLevelValue" instance-id="MXbNfa-lF1V3N" data-ps="af1c0deb-span-20" v-phw-setting="" class="au-target" au-target-id="138">
#             <a ph-tevent="job_click" ref="linkEle" href.bind="getUrl(linkEle, 'job', eachJob, '', eachJob.jobUrl)" data-ph-at-id="job-link" data-ps="af1c0deb-a-7" v-phw-setting="" class="au-target" au-target-id="139" ph-click-ctx="job" ph-tref="17578651096361503f" ph-tag="ph-search-results-v2" href="https://careers.roche.com/de/de/job/202203-110282/System-Development-Troubleshooter" data-ph-at-job-title-text="System Development Troubleshooter" data-ph-at-job-location-text="Rotkreuz, Zug, Schweiz" data-ph-at-job-location-area-text="Rotkreuz, Zug, Schweiz" data-ph-at-job-category-text="Manufacturing" data-access-list-item="0" data-ph-at-job-id-text="202203-110282" data-ph-at-job-type-text="Vollzeit" data-ph-at-job-industry-text="" data-ph-at-job-post-date-text="2022-03-08T00:00:00.000+0000" data-ph-at-job-seqno-text="ROCHGLOBAL202203110282EXTERNALDEDE" aria-label="System Development Troubleshooter Job-ID ist 202203-110282">
#                 <div class="job-title au-target phw-g-i-s7OoeY" data-ps="af1c0deb-div-34" v-phw-setting="" au-target-id="140">
#                     <!--anchor-->
#                     <!--anchor-->
#                     <span data-ps="af1c0deb-span-22" v-phw-setting="" class="au-target" au-target-id="146">System Development Troubleshooter </span>
#                 </div><!--anchor-->
#             </a>
#         </span>

#
# def collect_job_links(driver, wait, timeout=20, poll=0.5):
#     """
#     Scroll through results and collect unique job links (anchors with data-ph-at-id='job-link').
#     Returns a sorted list of hrefs.
#     """
#     end_time = time.time() + timeout
#     links = set()
#     last_count = -1
#
#     while time.time() < end_time:
#         elems = driver.find_elements(By.CSS_SELECTOR, "a[data-ph-at-id='job-link']")
#         for e in elems:
#             href = e.get_attribute("href")
#             if href:
#                 links.add(href)
#
#         if elems:
#             try:
#                 # scroll the last element into view to trigger lazy load / pagination
#                 driver.execute_script("arguments[0].scrollIntoView({block: 'end'}); window.scrollBy(0, -80);", elems[-1])
#             except Exception:
#                 pass
#
#         time.sleep(poll)
#
#         # stop early if no new links were found in the last iteration
#         if len(links) == last_count:
#             break
#         last_count = len(links)
#
#     return sorted(links)
#
# # usage: call after filters are applied and results rendered
# logger.info("Collecting job links from the results list")
# job_links = collect_job_links(driver, wait, timeout=20)
# logger.info(f"Collected {len(job_links)} job links")
# for link in job_links:
#     logger.info(link)

# Pagination "Next" button:

# <a href.bind="paginationUrls[currentSelectedPage + 1]" aria-label="Nächste Seite anzeigen" key-aria-label="viewNextPage" show.bind="nextButtonVisibility" ph-tevent="pagination_click" data-ph-tevent-attr-trait214="Next" data-ph-at-id="pagination-next-link" role="button" class="next-btn au-target" key-role="btnRole" data-ps="af1c0deb-a-11" v-phw-setting="" au-target-id="617" href="https://careers.roche.com/de/de/search-results?from=10&amp;s=1">
#                         <span data-ps="af1c0deb-span-135" v-phw-setting="" class="au-target" au-target-id="618">
#                             <ppc-content key="nextPaginationText" data-ph-at-id="pagination-next-text" data-ps="af1c0deb-ppc-content-15" v-phw-setting="" class="au-target" au-target-id="619">Nächster</ppc-content>
#                         </span>
#                         <span aria-hidden="true" class="icon icon-arrow-right au-target" data-ps="af1c0deb-span-136" v-phw-setting="" au-target-id="620"></span>
#                     </a>

# python
def collect_job_links_paginated(driver, wait, timeout=60, poll=0.5):
    """
    Iterate pages by clicking the pagination 'Next' link and collect unique job links
    (anchors with data-ph-at-id='job-link') from each page. Returns a sorted list of hrefs.
    """
    end_time = time.time() + timeout
    links = set()
    visited_pages = set()

    next_selector = "a[data-ph-at-id='pagination-next-link']"
    job_selector = "a[data-ph-at-id='job-link']"

    while time.time() < end_time:
        # collect links on current page
        elems = driver.find_elements(By.CSS_SELECTOR, job_selector)
        for e in elems:
            href = e.get_attribute("href")
            if href:
                links.add(href)

        current_url = driver.current_url
        # avoid infinite loops if page doesn't change
        if current_url in visited_pages:
            break
        visited_pages.add(current_url)

        # find next button
        next_elems = driver.find_elements(By.CSS_SELECTOR, next_selector)
        if not next_elems:
            break
        next_el = next_elems[0]

        # ensure next is interactable and has an href (otherwise assume no more pages)
        next_href = next_el.get_attribute("href")
        if not next_href:
            break

        # click next (try standard click, fallback to JS dispatch)
        try:
            try:
                next_el.click()
            except Exception:
                driver.execute_script(
                    "arguments[0].scrollIntoView({block: 'center'}); window.scrollBy(0, -80); arguments[0].click();",
                    next_el,
                )
        except Exception:
            # last resort: dispatch synthetic click event
            try:
                driver.execute_script(
                    "arguments[0].dispatchEvent(new MouseEvent('click', {bubbles:true,cancelable:true,view:window}));",
                    next_el,
                )
            except Exception:
                break

        # wait for navigation or new job links to appear
        start_wait = time.time()
        while time.time() - start_wait < 10:  # per-click wait (seconds)
            time.sleep(poll)
            # if URL changed, treat as success
            if driver.current_url != current_url:
                break
            # or if new job links loaded (different href set), treat as success
            new_hrefs = {e.get_attribute("href") for e in driver.find_elements(By.CSS_SELECTOR, job_selector) if e.get_attribute("href")}
            if not new_hrefs.issubset(links):
                break
        else:
            # timed out waiting for next page to load
            break

    return sorted(links)

logger.info("Collecting job links from paginated results")
job_links = collect_job_links_paginated(driver, wait, timeout=60)
logger.info(f"Collected {len(job_links)} job links across paginated results")
for link in job_links:
    logger.info(link)
# done
driver.quit()