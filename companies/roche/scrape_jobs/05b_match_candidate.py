import pandas as pd
import os
import time
from google import generativeai as genai
from google.generativeai.types import GenerateContentResponse, GenerationConfig
from pydantic import BaseModel, Field
from loguru import logger
import json

# --- Configuration ---
# Configure the generative AI model
# Make sure to set your GOOGLE_API_KEY environment variable, e.g. export GEMINI_API_KEY=`cat ~/api_key.txt`
try:
    genai.configure(api_key=os.environ["GOOGLE_API_KEY"])
    logger.info("Generative AI model configured successfully.")
except TypeError as e:
    logger.error(f"GOOGLE_API_KEY not set. Please set the environment variable. Details: {e}")
    exit()


INPUT_CSV_PATH = "df_with_ai_annotations.csv"
CANDIDATE_PROFILE_PATH = "candidate_profile.txt"
OUTPUT_CSV_PATH = "df_with_candidate_match.csv"
MAX_CHAR_LIMIT = 15000  # A safe character limit for the model prompt
SEPARATOR = "\n\n---\n\n"


# --- Pydantic Model for AI Output Validation ---
class CandidateMatch(BaseModel):
    """Defines the structure for the AI's response for each job."""
    match_score: int = Field(
        ...,
        description="An integer from 1 (poor match) to 5 (excellent match)."
    )
    idx: int = Field(
        ...,
        description="The original index of the job from the DataFrame."
    )


# --- Core Functions ---
def load_data(
        jobs_path: str,
        candidate_path: str
) -> tuple[pd.DataFrame | None, str | None]:
    """
    Loads the job annotations CSV and the candidate profile text file.

    Args:
        jobs_path (str): Path to the annotated jobs CSV file.
        candidate_path (str): Path to the candidate profile text file.

    Returns:
        A tuple containing the DataFrame and the candidate profile string,
        or (None, None) if a file cannot be found.
    """
    if not os.path.exists(jobs_path):
        logger.error(f"Input file not found: {jobs_path}")
        return None, None
    if not os.path.exists(candidate_path):
        logger.error(f"Candidate profile not found: {candidate_path}")
        return None, None

    try:
        df = pd.read_csv(jobs_path)
        logger.success(f"Successfully loaded {len(df)} records from {jobs_path}")
        with open(candidate_path, "r", encoding="utf-8") as f:
            candidate_profile = f.read()
        logger.success(f"Successfully loaded candidate profile from {candidate_path}")
        return df, candidate_profile
    except Exception as e:
        logger.error(f"Failed to load data: {e}")
        return None, None


def get_ai_match_rating(
        job_descriptions_chunk: str,
        candidate_profile: str
) -> GenerateContentResponse:
    """
    Sends a request to the generative AI to get a match score.

    Args:
        job_descriptions_chunk (str): A single string containing one or more
                                      job descriptions to be evaluated.
        candidate_profile (str): The text of the candidate's profile.

    Returns:
        The raw response object from the generative AI model.
    """
    model = genai.GenerativeModel("gemini-2.5-flash")
    prompt = f"""
    Based on the following candidate profile:
    ---CANDIDATE PROFILE---
    {candidate_profile}
    ---END CANDIDATE PROFILE---

    Please analyze each of the following job descriptions and provide a match score from 1 to 5,
    where 1 means a very poor match and 5 means an excellent match.
    The output must be a valid JSON object containing a list, where each item in the list
    corresponds to one of the job descriptions you analyzed.

    ---JOB DESCRIPTIONS---
    {job_descriptions_chunk}
    ---END JOB DESCRIPTIONS---
    """

    generation_config = GenerationConfig(
        response_mime_type="application/json",
        response_schema=list[CandidateMatch]
    )

    return model.generate_content(prompt, generation_config=generation_config)


def process_and_store_chunk(
        df: pd.DataFrame,
        entries: list[str],
        indices: list[int],
        candidate_profile: str,
        max_retries: int = 3,
        retry_delay: int = 5
) -> None:
    """
    Sends a chunk of job descriptions to the AI and stores the results.
    Includes a retry mechanism for handling transient API errors.

    Args:
        df (pd.DataFrame): The main DataFrame to update.
        entries (list[str]): The list of formatted job description strings.
        indices (list[int]): The list of original DataFrame indices.
        candidate_profile (str): The candidate's profile text.
        max_retries (int): The maximum number of times to retry a failed API call.
        retry_delay (int): The number of seconds to wait between retries.
    """
    if not entries:
        return

    job_descriptions_chunk = SEPARATOR.join(entries)
    logger.info(f"Sending chunk with {len(entries)} jobs for AI analysis (Indices: {indices[0]} to {indices[-1]}).")

    for attempt in range(max_retries):
        try:
            response = get_ai_match_rating(job_descriptions_chunk, candidate_profile)

            # The SDK can parse the JSON directly if the response is valid
            results = response.candidates[0].content.parts[0].text
            parsed_results = json.loads(results)

            for item in parsed_results:
                match = CandidateMatch(**item)
                df.loc[match.idx, "candidate_match_score"] = match.match_score
            logger.success(f"Successfully processed and stored results for {len(parsed_results)} jobs in the chunk.")
            return  # Exit the function on success

        except Exception as e:
            logger.warning(
                f"An error occurred on attempt {attempt + 1}/{max_retries} "
                f"for chunk starting with index {indices[0]}. Error: {e}"
            )
            if attempt < max_retries - 1:
                logger.info(f"Retrying in {retry_delay} seconds...")
                time.sleep(retry_delay)
            else:
                logger.error(f"Failed to process chunk after {max_retries} attempts. Indices: {indices}")


# --- Main Execution Block ---
if __name__ == "__main__":
    df_jobs, candidate_text = load_data(INPUT_CSV_PATH, CANDIDATE_PROFILE_PATH)

    if df_jobs is not None and candidate_text is not None:
        # Initialize the new column if it doesn't exist
        if "candidate_match_score" not in df_jobs.columns:
            df_jobs["candidate_match_score"] = None
            logger.info("Added 'candidate_match_score' column to the DataFrame.")

        # Prepare lists for batching jobs to send to the AI
        chunk_entries = []
        chunk_indices = []
        current_char_count = 0

        # Iterate over the DataFrame to process jobs in chunks
        for idx, row in df_jobs.iterrows():
            # Skip rows that have already been processed
            if pd.notna(row.get("candidate_match_score")):
                continue

            # Format the entry for the AI prompt
            entry = (
                f"idx: {idx}\n"
                f"title: {row.get('title', 'N/A')}\n"
                f"description: {row.get('description', 'N/A')}\n"
                f"job_summary: {row.get('job_summary', 'N/A')}"
            )

            entry_len = len(entry)
            separator_len = len(SEPARATOR)

            # If adding the new entry exceeds the limit, process the current chunk
            if chunk_entries and (current_char_count + entry_len + separator_len > MAX_CHAR_LIMIT):
                process_and_store_chunk(df_jobs, chunk_entries, chunk_indices, candidate_text)
                # Reset for the next chunk
                chunk_entries, chunk_indices, current_char_count = [], [], 0

            # Add the current job to the new chunk
            if entry_len <= MAX_CHAR_LIMIT:
                chunk_entries.append(entry)
                chunk_indices.append(idx)
                current_char_count += entry_len + (separator_len if len(chunk_entries) > 1 else 0)
            else:
                logger.warning(f"Skipping job index {idx} because its content exceeds the character limit of {MAX_CHAR_LIMIT}.")


        # Process any remaining jobs in the last chunk
        if chunk_entries:
            process_and_store_chunk(df_jobs, chunk_entries, chunk_indices, candidate_text)

        # Save the final DataFrame to a new CSV file
        try:
            df_jobs.to_csv(OUTPUT_CSV_PATH, index=False)
            logger.success(f"Processing complete. Final results saved to {OUTPUT_CSV_PATH}")
        except Exception as e:
            logger.error(f"Failed to save the final CSV file: {e}")