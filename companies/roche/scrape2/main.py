from helium import *
import shutil
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from loguru import logger

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
logger.info("Clicking on 'Schweiz' checkbox")
schweiz_css = "input[data-ph-at-text='Schweiz']"
try:
    schweiz_checkbox = wait.until(EC.element_to_be_clickable((By.CSS_SELECTOR, schweiz_css)))
    driver.execute_script("arguments[0].scrollIntoView({block: 'center', inline: 'nearest'});"
                          "window.scrollBy(0, -80);", schweiz_checkbox)
    try:
        schweiz_checkbox.click()
    except Exception:
        driver.execute_script("arguments[0].click();", schweiz_checkbox)
    logger.info("'Schweiz' checkbox clicked")
except Exception:
    logger.exception("Failed to click 'Schweiz' checkbox")
    driver.quit()
    exit(1)