from helium import *
import shutil
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from selenium import webdriver

# configure this browser binary: /usr/bin/google-chrome-stable
chromedriver_path = shutil.which("chromedriver") or "/usr/bin/chromedriver"
chrome_binary = shutil.which("google-chrome-stable") or "/usr/bin/google-chrome-stable"
options = Options()
options.binary_location = chrome_binary
service = Service(chromedriver_path)
driver = webdriver.Chrome(service=service, options=options)
set_driver(driver)

go_to("https://careers.roche.com/de/de/search-results")
