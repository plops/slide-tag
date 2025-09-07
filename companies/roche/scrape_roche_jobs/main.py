# -*- coding: utf-8 -*-
"""
This script scrapes job postings from the Roche careers website.

The target website (careers.roche.com) uses JavaScript to dynamically load job
listings. Therefore, this script utilizes the Selenium library to control a
web browser, which allows the JavaScript to execute and render the full HTML
content before scraping.

The script performs the following steps:
1.  Configures logging to output information to both the console and a file.
2.  Initializes a Selenium WebDriver to automate a Chrome browser.
3.  Navigates to the Roche careers search results page.
4.  Waits for the job listings to appear.
5.  Iteratively scrapes the URLs of all job postings, clicking the "Next"
    button to handle pagination.
6.  Once all job URLs are collected, it visits each URL individually to
    scrape the full job description.
7.  Saves the collected data (URL and description) into a CSV file.

Prerequisites:
- Python 3.x
- Libraries: selenium, beautifulsoup4, pandas
  (install with: pip install selenium beautifulsoup4 pandas)
- A Selenium WebDriver for your browser (e.g., chromedriver for Chrome)
  must be downloaded and accessible in your system's PATH or specified
  in the script.
"""

import logging
import logging.handlers
import os
import time

import pandas as pd
import requests
from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.common.exceptions import NoSuchElementException, TimeoutException
from selenium.webdriver.common.by import By
from selenium.webdriver.chrome.service import Service as ChromeService
from selenium.webdriver.chrome.options import Options
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import WebDriverWait

def configure_logging(level=logging.INFO, log_dir="logs", log_file="roche_scrape.log"):
    """
    Configures a logger to output to the console and a rotating file.

    Args:
        level (int): The logging level (e.g., logging.INFO).
        log_dir (str): The directory to store log files.
        log_file (str): The name of the log file.

    Returns:
        logging.Logger: The configured logger instance.
    """
    if not os.path.exists(log_dir):
        os.makedirs(log_dir, exist_ok=True)

    logger = logging.getLogger("scrape_roche_jobs")
    logger.setLevel(level)
    # Prevent duplicate handlers if the function is called multiple times
    if logger.hasHandlers():
        logger.handlers.clear()

    formatter = logging.Formatter(
        "%(asctime)s %(levelname)s %(name)s - %(message)s", "%Y-%m-%d %H:%M:%S"
    )

    # Console handler for real-time feedback
    ch = logging.StreamHandler()
    ch.setLevel(level)
    ch.setFormatter(formatter)
    logger.addHandler(ch)

    # Rotating file handler for persistent logs
    fh = logging.handlers.RotatingFileHandler(
        os.path.join(log_dir, log_file), maxBytes=5 * 1024 * 1024, backupCount=3, encoding="utf-8"
    )
    fh.setLevel(level)
    fh.setFormatter(formatter)
    logger.addHandler(fh)

    return logger

def scrape_all_job_links(base_url, driver):
    """
    Uses Selenium to navigate through all job pages and collect job links.

    Args:
        base_url (str): The starting URL for the job search results.
        driver (webdriver): The Selenium WebDriver instance.

    Returns:
        list: A list of unique URLs for all found job postings.
    """
    logger = logging.getLogger("scrape_roche_jobs")
    job_links = set()  # Use a set to automatically handle duplicate links
    page_count = 1

    driver.get(base_url)

    while True:
        logger.info("Scraping job links from page %s...", page_count)

        # Wait for the job list container to be present and for links to be visible.
        # This is a crucial step to ensure the JavaScript has finished rendering.
        try:
            WebDriverWait(driver, 20).until(
                EC.presence_of_element_located((By.CSS_SELECTOR, "a.job-title-link"))
            )
        except TimeoutException:
            logger.warning("Timed out waiting for job listings on page %s. It might be the end.", page_count)
            break

        # Parse the page source with BeautifulSoup after it's fully loaded
        soup = BeautifulSoup(driver.page_source, "html.parser")
        current_page_links = soup.select("a.job-title-link")

        if not current_page_links:
            logger.info("No job listings found on page %s, stopping.", page_count)
            break

        # Extract hrefs and add them to the set
        for link in current_page_links:
            if link.has_attr('href'):
                full_url = "https://careers.roche.com" + link['href']
                job_links.add(full_url)

        logger.info("Found %s new links on this page. Total unique links: %s.", len(current_page_links), len(job_links))

        # --- Pagination Logic ---
        try:
            # Find the "Next" button element
            next_button = driver.find_element(By.CSS_SELECTOR, 'a[data-ph-at-id="pagination-next-link"]')

            # Check if the button is disabled (often indicated by a specific attribute or class)
            if 'disabled' in next_button.get_attribute('class') or not next_button.is_enabled():
                logger.info("The 'Next' button is disabled. Reached the last page.")
                break

            # Use JavaScript to click the button to avoid potential interception issues
            driver.execute_script("arguments[0].click();", next_button)
            page_count += 1
            # Wait for the next page to load; a small static wait can be helpful here
            # A more robust solution would be to wait for an element on the next page to change
            time.sleep(3)

        except NoSuchElementException:
            logger.info("No 'Next' button found. Reached the last page.")
            break
        except Exception as e:
            logger.error("An error occurred during pagination: %s", e, exc_info=True)
            break

    return list(job_links)

def scrape_job_descriptions(job_links):
    """
    Scrapes the description for each job from its individual URL.

    Args:
        job_links (list): A list of URLs for the job postings.

    Returns:
        list: A list of dictionaries, where each dictionary contains the
              job's URL and its full text description.
    """
    logger = logging.getLogger("scrape_roche_jobs")
    job_details = []

    for link in job_links:
        logger.info("Scraping job description from: %s", link)
        try:
            # Using requests here is faster than using Selenium for each individual page
            response = requests.get(link, timeout=15)
            response.raise_for_status()  # Raise an exception for bad status codes

            soup = BeautifulSoup(response.content, "html.parser")
            # The job description is within a div with the class 'jd-info'
            description_div = soup.find("div", class_="jd-info")

            if description_div:
                # Use a separator for better readability and structure
                description_text = description_div.get_text(separator='\n').strip()
                job_details.append({
                    "url": link,
                    "description": description_text
                })
            else:
                logger.warning("No description div ('jd-info') found for URL: %s", link)

        except requests.exceptions.RequestException as e:
            logger.error("Error fetching job description from %s: %s", link, e, exc_info=True)
            continue # Move to the next link

    return job_details

def main():
    """
    Main function to orchestrate the scraping process.
    """
    # Configure the logger at the start of the execution
    logger = configure_logging()

    base_url = "https://careers.roche.com/de/de/search-results"

    # --- Selenium WebDriver Setup ---
    logger.info("Initializing Selenium WebDriver...")
    try:
        chrome_options = Options()
        chrome_options.add_argument("--headless")  # Run browser in the background
        chrome_options.add_argument("--no-sandbox")
        chrome_options.add_argument("--disable-dev-shm-usage")
        chrome_options.add_argument("user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")

        # webdriver-manager automatically downloads and manages the driver
        service = ChromeService(ChromeDriverManager().install())
        driver = webdriver.Chrome(service=service, options=chrome_options)

    except Exception as e:
        logger.error("Failed to initialize Selenium WebDriver: %s", e, exc_info=True)
        return

    # --- Start Scraping ---
    try:
        job_links = scrape_all_job_links(base_url, driver)
    finally:
        # Ensure the browser is closed even if errors occur
        logger.info("Closing the WebDriver.")
        driver.quit()

    if not job_links:
        logger.warning("No job links were found. Exiting.")
        return

    logger.info("Successfully collected %s unique job links. Now scraping details...", len(job_links))
    jobs_data = scrape_job_descriptions(job_links)

    if jobs_data:
        # Create a pandas DataFrame and save the results to a CSV file
        df = pd.DataFrame(jobs_data)
        csv_path = "roche_job_descriptions.csv"
        try:
            df.to_csv(csv_path, index=False, encoding='utf-8-sig')
            logger.info("Scraping finished. Data for %s jobs saved to %s", len(df), csv_path)
        except Exception as e:
            logger.error("Failed to save data to CSV file %s: %s", csv_path, e, exc_info=True)
    else:
        logger.warning("No job descriptions could be scraped.")


if __name__ == '__main__':
    main()