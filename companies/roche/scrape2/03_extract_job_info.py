import sys
import json
import re
from bs4 import BeautifulSoup

def extract_job_description(html_content):
    """
    Parses the HTML to find the job description embedded in a script tag.
    """
    soup = BeautifulSoup(html_content, 'html.parser')

    # Find all script tags
    script_tags = soup.find_all('script')

    job_data_json = None

    # Loop through scripts to find the one containing the job data
    for script in script_tags:
        # The job data is in a script that defines 'phApp.ddo'
        if script.string and 'phApp.ddo' in script.string:
            # Use regex to find the JSON object assigned to phApp.ddo
            match = re.search(r'phApp\.ddo\s*=\s*(\{.*?\});', script.string, re.DOTALL)
            if match:
                job_data_json = match.group(1)
                break

    if not job_data_json:
        return "Could not find the job data JSON in the script tags."

    try:
        # Load the JSON data into a Python dictionary
        data = json.loads(job_data_json)

        # Navigate through the dictionary to get the job description HTML
        # The path is jobDetail -> data -> job -> description
        description_html = data.get('jobDetail', {}).get('data', {}).get('job', {}).get('description', '')

        if not description_html:
            return "Job description field was empty in the JSON data."

        # Now, parse the extracted description HTML to get clean text
        description_soup = BeautifulSoup(description_html, 'html.parser')

        # Get text from the parsed HTML, use a separator to ensure space between elements
        clean_text = description_soup.get_text(separator='\n', strip=True)

        return clean_text

    except json.JSONDecodeError:
        return "Failed to decode JSON from the script tag."
    except KeyError as e:
        return f"Could not find expected key in JSON data: {e}"

def main():
    """
    Main function to run the script from the command line.
    """
    # Check if a filename is provided as a command-line argument
    if len(sys.argv) < 2:
        print("Usage: python extract_job_info.py <filename.html>")
        sys.exit(1)

    filename = sys.argv[1]

    try:
        with open(filename, 'r', encoding='utf-8') as f:
            html_content = f.read()

        relevant_text = extract_job_description(html_content)
        print(relevant_text)

    except FileNotFoundError:
        print(f"Error: The file '{filename}' was not found.")
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()