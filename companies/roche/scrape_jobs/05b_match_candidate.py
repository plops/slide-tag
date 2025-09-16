import pandas as pd
import os
import time
from google import genai
from google.genai import types
from pydantic import BaseModel, Field
from loguru import logger

# --- Configuration ---
INPUT_CSV_PATH = "df_with_ai_annotations.csv"
CANDIDATE_PROFILE_PATH = "candidate_profile.txt"
OUTPUT_CSV_PATH = "df_with_candidate_match.csv"
MAX_WORD_LIMIT = 15000  # The maximum number of words per API request
SEPARATOR = "\n\n---\n\n"
MODEL_NAME = "gemini-1.5-flash" # Or another suitable model like "gemini-pro"

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
        # Use the first column as the index since it was the original index
        df = pd.read_csv(jobs_path, index_col=0)
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
):
    """
    Sends a request to the generative AI to get a match score using the genai.Client.

    Args:
        job_descriptions_chunk (str): A single string containing one or more
                                      job descriptions to be evaluated.
        candidate_profile (str): The text of the candidate's profile.

    Returns:
        The parsed response object from the generative AI model.
    """
    api_key = os.environ.get("GEMINI_API_KEY")
    if not api_key:
        raise ValueError("GEMINI_API_KEY environment variable not set.")

    client = genai.Client(api_key=api_key)

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

    contents = [
        types.Content(
            role="user",
            parts=[types.Part.from_text(text=prompt)],
        ),
    ]

    generation_config = types.GenerateContentConfig(
        response_mime_type="application/json",
        response_schema=list[CandidateMatch],
    )

    result = client.models.generate_content(
        model=MODEL_NAME,
        contents=contents,
        config=generation_config,
    )
    return result


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

    word_count = len(job_descriptions_chunk.split())
    logger.info(
        f"Sending chunk with {len(entries)} jobs ({word_count} words) for AI analysis "
        f"(Indices: {indices[0]} to {indices[-1]})."
    )

    for attempt in range(max_retries):
        try:
            response = get_ai_match_rating(job_descriptions_chunk, candidate_profile)
            parsed_results = response.parsed

            if not parsed_results:
                logger.warning(f"AI returned an empty result for chunk starting with index {indices[0]}.")
                return

            for match in parsed_results:
                df.loc[match.idx, "candidate_match_score"] = match.match_score
            logger.success(f"Successfully processed and stored results for {len(parsed_results)} jobs in the chunk.")
            return

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
    if not os.environ.get("GEMINI_API_KEY"):
        logger.error("GEMINI_API_KEY not set. Please set the environment variable before running.")
    else:
        df_jobs, candidate_text = load_data(INPUT_CSV_PATH, CANDIDATE_PROFILE_PATH)

        if df_jobs is not None and candidate_text is not None:
            if "candidate_match_score" not in df_jobs.columns:
                df_jobs["candidate_match_score"] = None
                logger.info("Added 'candidate_match_score' column to the DataFrame.")

            chunk_entries = []
            chunk_indices = []
            current_word_count = 0

            for idx, row in df_jobs.iterrows():
                if pd.notna(row.get("candidate_match_score")):
                    continue

                entry = (
                    f"idx: {idx}\n"
                    f"title: {row.get('title', 'N/A')}\n"
                    # f"description: {row.get('description', 'N/A')}\n"
                    f"job_summary: {row.get('job_summary', 'N/A')}"
                )

                # Calculate word count for the current entry
                entry_word_count = len(entry.split())

                # If adding the new entry exceeds the word limit, process the current chunk
                if chunk_entries and (current_word_count + entry_word_count > MAX_WORD_LIMIT):
                    process_and_store_chunk(df_jobs, chunk_entries, chunk_indices, candidate_text)
                    # Reset for the next chunk
                    chunk_entries, chunk_indices, current_word_count = [], [], 0

                # Add the current job to the new chunk, checking its individual size
                if entry_word_count <= MAX_WORD_LIMIT:
                    chunk_entries.append(entry)
                    chunk_indices.append(idx)
                    current_word_count += entry_word_count
                else:
                    logger.warning(
                        f"Skipping job index {idx} because its content ({entry_word_count} words) "
                        f"exceeds the word limit of {MAX_WORD_LIMIT}."
                    )

            # Process any remaining jobs in the last chunk
            if chunk_entries:
                process_and_store_chunk(df_jobs, chunk_entries, chunk_indices, candidate_text)

            try:
                df_jobs.to_csv(OUTPUT_CSV_PATH, index=True)
                logger.success(f"Processing complete. Final results saved to {OUTPUT_CSV_PATH}")
            except Exception as e:
                logger.error(f"Failed to save the final CSV file: {e}")