import requests
from bs4 import BeautifulSoup
import pandas as pd
import logging
import logging.handlers
import os

def configure_logging(level=logging.INFO, log_dir="logs", log_file="roche_scrape.log"):
    """
    Configure logging: console + rotating file handler.
    """
    if not os.path.exists(log_dir):
        os.makedirs(log_dir, exist_ok=True)

    logger = logging.getLogger("scrape_roche_jobs")
    logger.setLevel(level)
    # Remove any existing handlers (useful when reloading)
    logger.handlers = []

    formatter = logging.Formatter(
        "%(asctime)s %(levelname)s %(name)s - %(message)s", "%Y-%m-%d %H:%M:%S"
    )

    # Console handler
    ch = logging.StreamHandler()
    ch.setLevel(level)
    ch.setFormatter(formatter)
    logger.addHandler(ch)

    # Rotating file handler
    fh = logging.handlers.RotatingFileHandler(
        os.path.join(log_dir, log_file), maxBytes=5 * 1024 * 1024, backupCount=3, encoding="utf-8"
    )
    fh.setLevel(level)
    fh.setFormatter(formatter)
    logger.addHandler(fh)

    return logger

def scrape_roche_jobs():
    """
    Scrapes all job descriptions from the Roche careers website.

    Returns:
        list: A list of dictionaries, where each dictionary represents a job
              and contains the job's URL and description.
    """
    logger = logging.getLogger("scrape_roche_jobs")

    base_url = "https://careers.roche.com/de/de/search-results"
    job_links = []
    page = 1
    while True:
        logger.info("Scraping page %s...", page)
        if page == 1:
            url = base_url
        else:
            url = f"{base_url}?from={(page-1)*10}&s=1"

        try:
            response = requests.get(url)
            response.raise_for_status()  # Raise an exception for bad status codes
        except requests.exceptions.RequestException as e:
            logger.error("Error fetching page %s: %s", page, e, exc_info=True)
            break

        soup = BeautifulSoup(response.content, "html.parser")

        # Find all job links on the page
        job_listings = soup.select("a.job-title-link")

        if not job_listings:
            logger.info("No job listings found on page %s, stopping.", page)
            break

        for job in job_listings:
            if job.has_attr('href'):
                job_links.append("https://careers.roche.com" + job['href'])

        logger.debug("Found %s job links so far.", len(job_links))

        # Check for the next page link
        next_button = soup.select_one('a[data-ph-at-id="pagination-next-link"]')
        if not next_button or 'disabled' in next_button.attrs:
            logger.info("No next page found, finished paging.")
            break

        page += 1

    job_descriptions = []
    for link in job_links:
        logger.info("Scraping job description from: %s", link)
        try:
            response = requests.get(link)
            response.raise_for_status()
        except requests.exceptions.RequestException as e:
            logger.error("Error fetching job description from %s: %s", link, e, exc_info=True)
            continue

        soup = BeautifulSoup(response.content, "html.parser")
        job_description = soup.find("div", class_="jd-info")

        if job_description:
            job_descriptions.append({
                "url": link,
                "description": job_description.get_text(separator='\\n').strip()
            })
        else:
            logger.debug("No job description found for %s", link)

    return job_descriptions

if __name__ == '__main__':
    # Configure logger once at startup
    logger = configure_logging()

    jobs = scrape_roche_jobs()

    if jobs:
        for i, job in enumerate(jobs):
            logger.info("--- Job %s ---", i+1)
            logger.info("URL: %s", job['url'])
            logger.debug("Description: %s", job['description'][:200].replace('\n',' '))  # first 200 chars as debug

        # Create a DataFrame and save to CSV
        df = pd.DataFrame(jobs)
        csv_path = "roche_job_descriptions.csv"
        try:
            df.to_csv(csv_path, index=False)
            logger.info("Scraping finished. Data saved to %s", csv_path)
        except Exception as e:
            logger.error("Failed to save CSV %s: %s", csv_path, e, exc_info=True)
    else:
        logger.info("No jobs found or an error occurred during scraping.")
