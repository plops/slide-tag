import pandas as pd
import sqlite3
import os

def filter_jobs_data(db_path='jobs_minutils.db'):
    """
    Loads the Jobs table from a SQLite database, filters it based on specific
    criteria, and returns the resulting pandas DataFrame.

    Args:
        db_path (str): The path to the SQLite database file.

    Returns:
        pandas.DataFrame: A DataFrame containing the filtered job data,
                          or None if the database/table cannot be accessed.
    """
    # --- 1. Input Validation ---
    if not os.path.exists(db_path):
        print(f"Error: Database file not found at '{db_path}'")
        return None

    try:
        # --- 2. Database Connection and Data Loading ---
        # Create a connection to the SQLite database
        conn = sqlite3.connect(db_path)
        print(f"Successfully connected to {db_path}")

        # Use pandas to read the entire 'Jobs' table into a DataFrame
        # The connection is automatically closed by pandas after reading
        df = pd.read_sql_query("SELECT * FROM Jobs", conn)
        print(f"Successfully loaded {len(df)} records from the 'Jobs' table.")

    except (sqlite3.Error, pd.errors.DatabaseError) as e:
        print(f"Error accessing the database or table: {e}")
        return None
    finally:
        # Ensure the connection is closed
        if 'conn' in locals() and conn:
            conn.close()

    # --- 3. Filtering Logic ---
    print("\nApplying filters...")

    # Create a copy to avoid SettingWithCopyWarning
    filtered_df = df.copy()

    # Filter 1: 'job_family' is not 'Internship'
    # Using .ne() which is equivalent to !=
    initial_count = len(filtered_df)
    filtered_df = filtered_df[filtered_df['job_family'].ne('Internship')]
    print(f"- Excluded {initial_count - len(filtered_df)} jobs with job_family 'Internship'.")

    # Filter 2: 'job_family' does not start with 'Food'
    # The ~ operator inverts the boolean mask
    initial_count = len(filtered_df)
    # Using .str.startswith() for robust matching and handling potential missing values (na=False)
    filtered_df = filtered_df[~filtered_df['job_family'].str.startswith('Food', na=False)]
    print(f"- Excluded {initial_count - len(filtered_df)} jobs where job_family starts with 'Food'.")


    # Filter 3: 'job_profile' does not contain 'finance' (case-insensitive)
    initial_count = len(filtered_df)
    # Using .str.contains() with case=False for case-insensitive search
    # na=False ensures that rows with a missing job_profile are kept
    filtered_df = filtered_df[~filtered_df['job_profile'].str.contains('finance', case=False, na=False)]
    print(f"- Excluded {initial_count - len(filtered_df)} jobs with 'finance' in job_profile.")

    # Exclude job_level Executive
    initial_count = len(filtered_df)
    filtered_df = filtered_df[filtered_df['job_level'].ne('Executive')]
    print(f"- Excluded {initial_count - len(filtered_df)} jobs with job_level 'Executive'.")


    print(f"\nFiltering complete. {len(filtered_df)} jobs remain.")
    return filtered_df

# --- 4. Main Execution Block ---
# if __name__ == "__main__":
# Specify the database file name
database_file = 'jobs_minutils.db'

# Run the filtering function
df = filter_jobs_data(database_file)

# Display the results if the filtering was successful
if df is not None:
    print("\n--- Filtered Job Data ---")
    # Configure pandas to display more columns if needed
    pd.set_option('display.max_columns', None)
    pd.set_option('display.width', 1000)

    # Display the first 20 rows of the final DataFrame
    print(df.head(20))

    # You can also save the filtered data to a new file, for example:
    # final_jobs_df.to_csv('filtered_jobs.csv', index=False)
