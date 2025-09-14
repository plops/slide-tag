import sys
import re
import os

def extract_and_save_json(html_filepath):
    """
    Extracts a JSON object from a script tag in an HTML file and saves it.

    Args:
        html_filepath (str): The path to the input HTML file.
    """
    try:
        with open(html_filepath, 'r', encoding='utf-8') as f:
            html_content = f.read()

        # Regex to find the JavaScript object assigned to phApp.ddo
        # re.DOTALL is crucial because the JSON string contains newlines
        match = re.search(r'phApp\.ddo\s*=\s*(\{.*?\});', html_content, re.DOTALL)

        if not match:
            print(f"Error: Could not find the 'phApp.ddo' JSON object in {html_filepath}")
            return

        # The first capturing group contains the JSON string
        json_string = match.group(1)

        # Create the output filename by replacing .html with .json
        base_name = os.path.splitext(html_filepath)[0]
        output_filepath = base_name + ".json"

        # Write the extracted JSON string to the new file
        with open(output_filepath, 'w', encoding='utf-8') as f:
            f.write(json_string)

        print(f"Successfully extracted JSON to: {output_filepath}")

    except FileNotFoundError:
        print(f"Error: The file '{html_filepath}' was not found.")
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        sys.exit(1)


def main():
    """
    Main function to run the script from the command line.
    """
    if len(sys.argv) < 2:
        print("Usage: python extract_json_from_html.py <filename.html>")
        sys.exit(1)

    filename = sys.argv[1]
    extract_and_save_json(filename)


if __name__ == '__main__':
    main()