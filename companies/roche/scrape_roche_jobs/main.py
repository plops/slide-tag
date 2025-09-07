import requests
from bs4 import BeautifulSoup
import pandas as pd

def scrape_roche_jobs():
    """
    Scrapes all job descriptions from the Roche careers website.

    Returns:
        list: A list of dictionaries, where each dictionary represents a job
              and contains the job's URL and description.
    """
    base_url = "https://careers.roche.com/de/de/search-results"
    job_links = []
    page = 1
    while True:
        print(f"Scraping page {page}...")
        if page == 1:
            url = base_url
        else:
            url = f"{base_url}?from={(page-1)*10}&s=1"

        try:
            response = requests.get(url)
            response.raise_for_status()  # Raise an exception for bad status codes
        except requests.exceptions.RequestException as e:
            print(f"Error fetching page {page}: {e}")
            break

        soup = BeautifulSoup(response.content, "html.parser")

        # Find all job links on the page
        job_listings = soup.select("a.job-title-link")

        if not job_listings:
            break

        for job in job_listings:
            if job.has_attr('href'):
                job_links.append("https://careers.roche.com" + job['href'])

        # Check for the next page link
        next_button = soup.select_one('a[data-ph-at-id="pagination-next-link"]')
        if not next_button or 'disabled' in next_button.attrs:
            break

        page += 1

    job_descriptions = []
    for link in job_links:
        print(f"Scraping job description from: {link}")
        try:
            response = requests.get(link)
            response.raise_for_status()
        except requests.exceptions.RequestException as e:
            print(f"Error fetching job description from {link}: {e}")
            continue

        soup = BeautifulSoup(response.content, "html.parser")
        job_description = soup.find("div", class_="jd-info")

        if job_description:
            job_descriptions.append({
                "url": link,
                "description": job_description.get_text(separator='\\n').strip()
            })

    return job_descriptions

if __name__ == '__main__':
    jobs = scrape_roche_jobs()

    if jobs:
        for i, job in enumerate(jobs):
            print(f"\\n--- Job {i+1} ---")
            print(f"URL: {job['url']}")
            print(f"Description:\\n{job['description']}")

        # Create a DataFrame and save to CSV
        df = pd.DataFrame(jobs)
        df.to_csv("roche_job_descriptions.csv", index=False)
        print("\\nScraping finished. Data saved to roche_job_descriptions.csv")
    else:
        print("\\nNo jobs found or an error occurred during scraping.")
