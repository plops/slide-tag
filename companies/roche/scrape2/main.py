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

# click away the cookie banner:
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

# click on the checkbox for Schweiz:
logger.info("Attempting to select the 'Schweiz' country filter")
try:
    label = wait.until(EC.element_to_be_clickable((By.CSS_SELECTOR, 'label[for="country_phs_46"]')))
    logger.info("Found Schweiz label, clicking via JS")
    driver.execute_script("arguments[0].click();", label)
    logger.info("Clicked Schweiz label successfully")
except Exception:
    logger.exception("Failed to click Schweiz label, falling back to clicking the input checkbox directly")
    # fallback: click the input checkbox directly
    try:
        checkbox = wait.until(EC.element_to_be_clickable((By.ID, "country_phs_46")))
        logger.info("Found Schweiz checkbox input, clicking via JS")
        driver.execute_script("arguments[0].click();", checkbox)
        logger.info("Clicked Schweiz checkbox input successfully")
    except Exception:
        logger.exception("Failed to click Schweiz checkbox input; giving up on selecting the filter")
