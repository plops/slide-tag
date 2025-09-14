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
    filtered_df = filtered_df[~filtered_df['job_family'].str.contains('treasury', case=False, na=False)]
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

# # Display the results if the filtering was successful
# if df is not None:
#     print("\n--- Filtered Job Data ---")
#     # Configure pandas to display more columns if needed
#     pd.set_option('display.max_columns', None)
#     pd.set_option('display.width', 1000)
#
#     # Display the first 20 rows of the final DataFrame
#     print(df.head(20))
#
#     # You can also save the filtered data to a new file, for example:
#     # final_jobs_df.to_csv('filtered_jobs.csv', index=False)

# There are more jobs than supervisors.
# >>> len(df.supervisory_organization.unique())
# 103
# >>> len(df.title.unique())
# 120
# Sort supervisors by number of jobs

def sort_supervisors_by_job_count(df: pd.DataFrame, top_n: int | None = None, save_path: str | None = None) -> pd.DataFrame:
    """
    Return a DataFrame of supervisors sorted by number of jobs (descending).
    - Replaces missing supervisory values with 'MISSING' so they are counted.
    - Adds a relative percentage column.
    - Optionally returns only top_n rows and saves to CSV if save_path is provided.
    """
    if df is None or 'supervisory_organization' not in df.columns:
        raise ValueError("DataFrame must contain a 'supervisory_organization' column")

    counts = (
        df['supervisory_organization']
        .fillna('MISSING')
        .value_counts(dropna=False)
        .rename_axis('supervisory_organization')
        .reset_index(name='job_count')
    )

    counts['pct_of_total'] = counts['job_count'] / counts['job_count'].sum()

    if top_n is not None:
        counts = counts.head(top_n)

    if save_path:
        counts.to_csv(save_path, index=False)

    return counts

# Example usage:
counts_df = sort_supervisors_by_job_count(df, top_n=20, save_path='supervisor_job_counts.csv')

#                              supervisory_organization  job_count  pct_of_total
# 0   DSRMGJ NAP/qPCR & Sequencing (Vahid Akbarzadeh...          5      0.040984
# 1      MMED Data Backbone (Dominik Wendel) (50407023)          5      0.040984
# 2   MMMCFC7D Technical Asset Care GEF (Florian Sän...          3      0.024590
# 3   EFHCBO Basel Site Services Chapter H14 (Philip...          2      0.016393
# 4   PRE Nucleic Acid Based Medicine (Hendrik Knötg...          2      0.016393
# 5   MMEF Operational Excellence (Eric Auschitzky) ...          2      0.016393
# 6    PNUF Immunosafety 1 (Donata De Marco) (50524407)          2      0.016393
# 7   PAB CVM in vitro Research (Norbert Tennagels) ...          2      0.016393
# 8   CA Digital, Campaigns, Brand & Creative (Matt ...          2      0.016393
# 9   MMDPF Global Process Engineering and Manufactu...          2      0.016393
# 10  PREBB Oligonucleotide Research (Johannes Braun...          2      0.016393
# 11       MMMCGB Calibration (Ali Üstündag) (50669427)          2      0.016393
# 12  PRDF Lead Discovery (Federica Morandi) (50225496)          1      0.008197
# 13  MMDZAB Device Engineering Section B (Eldin Sma...          1      0.008197

# Which jobs might be relevant to Slide-tag?

# DSRMGJ NAP/qPCR & Sequencing (Vahid Akbarzadeh...): This group is highly relevant. The name explicitly mentions "Sequencing," which is a core component of the Slide-tag workflow. The "NAP/qPCR" likely refers to Nucleic Acid Preparation and quantitative Polymerase Chain Reaction, which are foundational molecular biology techniques. A team focused on sequencing would be directly involved in either developing or utilizing technologies like Slide-tag.
# PRE Nucleic Acid Based Medicine (Hendrik Knötg...) and PREBB Oligonucleotide Research (Johannes Braun...): Both of these "PRE" groups are very likely part of pRED (Pharma Research and Early Development).[11][12] "Nucleic Acid Based Medicine" and "Oligonucleotide Research" are directly related to the molecular components of Slide-tag, which uses DNA barcodes (oligonucleotides) to spatially index nucleic acids in tissue samples. These teams would be at the forefront of developing and applying such novel research tools for therapeutic discovery.
# MMED Data Backbone (Dominik Wendel): This organization is also highly relevant, with "MMED" possibly standing for Molecular Medicine or a similar data-focused division. The "Data Backbone" designation strongly suggests a bioinformatics and data science group. A significant part of the Slide-tag workflow is the computational analysis of large sequencing datasets to reconstruct the spatial information.[6] This group would likely be responsible for developing the algorithms and infrastructure to handle and interpret Slide-tag data.

# Of the organizations listed, the ones most relevant to Slide-tag and similar spatial genomics technologies are those involved in **early-stage research (pRED/gRED), genomics, sequencing, pathology, and computational biology/data science**.
#
# Here is a selection of the most relevant organizations, categorized by their likely function related to Slide-tag technology.
#
# ### Tier 1: Most Directly Relevant (Core Technology & Platforms)
#
# These groups are likely responsible for developing, running, or directly managing the platforms and core molecular biology workflows for technologies like Slide-tag.
#
# *   **'DSRMGJ NAP/qPCR & Sequencing (Vahid Akbarzadeh) (32410074)'**: The name explicitly includes **"Sequencing,"** which is the ultimate readout for Slide-tag. This group would handle the core instrumentation and data generation.
# *   **'PSTB Genomics 360 Lab (Kim Schneider) (50473535)'**: A **"Genomics Lab"** is a perfect fit. The "360" suggests a focus on comprehensive, multi-omic approaches, which is exactly what spatial technologies enable.
# *   **'PNUA Pathology 1 (Björn Jacobsen) (32231909)'**: **"Pathology"** is the discipline being revolutionized by spatial genomics. This group would be responsible for the tissue handling, preparation, and histopathological interpretation that is fundamental to Slide-tag.
# *   **'PRE Nucleic Acid Based Medicine (Hendrik Knötgen) (50211838)'**: "PRE" indicates **Pharma Research and Early Development (pRED)**. "Nucleic Acid Based Medicine" is directly related to the core components of Slide-tag (RNA and DNA barcodes).
# *   **'PREBB Oligonucleotide Research (Johannes Braun) (50667061)'**: Also in pRED, this group's focus on **"Oligonucleotide Research"** is highly relevant, as the barcode technology at the heart of Slide-tag relies on custom DNA oligonucleotides.
# *   **'PREB Therapeutic Oligonucleotides (Felix Schumacher) (50467570)'**: Similar to the above, this pRED group's expertise in oligonucleotides is critical to the chemical and molecular aspects of the technology.
#
# ### Tier 2: Highly Relevant (Data Analysis & Application)
#
# These groups would be the primary users and interpreters of Slide-tag data, using it for drug discovery, target identification, and understanding disease biology.
#
# *   **'GTA Analytics (Fabian Birzele) (50606589)'**: A general **"Analytics"** group in a research context would be responsible for processing and interpreting the complex data generated by Slide-tag.
# *   **'GTAE Computational CoE (Jörg Degen) (50682980)'**: A **"Computational Center of Excellence"** is a prime candidate for developing the sophisticated algorithms required for spatial reconstruction and data analysis.
# *   **'GSFHB Computational Catalyst (Jens Reeder) (30931809)'**: The name **"Computational Catalyst"** strongly implies a bioinformatics group focused on driving research forward using advanced computational methods.
# *   **'GSAA Prescient AI ML (Vladimir Gligorijevic) (50364410)'**: This group's focus on **"AI/ML"** is crucial for analyzing the massive, high-dimensional datasets from spatial genomics to identify novel patterns and biomarkers.
# *   **'GSAG Prescient Frontier Research (Stephen Ra) (50427018)'**: **"Frontier Research"** is precisely where cutting-edge technologies like Slide-tag are first adopted and utilized to break new scientific ground.
# *   **'MMED Data Backbone (Dominik Wendel) (50407023)'**: A **"Data Backbone"** group is essential for managing, processing, and providing access to the large-scale datasets produced.
# *   **'PRDF Lead Discovery (Federica Morandi) (50225496)'**: **"Lead Discovery"** is a core part of early pharma R&D where this technology would be used to identify and validate new drug targets.
# *   **'POR Discovery Oncology (Ashley Lakner) (25696339)'**: A key therapeutic area. **"Discovery Oncology"** would be a major user of Slide-tag to study the tumor microenvironment.
# *   **'TNDAB Neurodegeneration (Christopher Lane) (50663284)'**: Another key application area. This group would use spatial genomics to map cellular changes in diseases like Alzheimer's.
# *   **'PCE Early Development (Luka Kulic) (50310171)'**: **"Early Development"** is the phase where understanding a drug's mechanism of action in tissue is critical, a key application for Slide-tag.

# make a list of interesting organizations
orgs = [
    'DSRMGJ NAP/qPCR & Sequencing (Vahid Akbarzadeh) (32410074)',
    'PSTB Genomics 360 Lab (Kim Schneider) (50473535)',
    'PNUA Pathology 1 (Björn Jacobsen) (32231909)',
    'PRE Nucleic Acid Based Medicine (Hendrik Knötgen) (50211838)',
    'PREBB Oligonucleotide Research (Johannes Braun) (50667061)',
    'PREB Therapeutic Oligonucleotides (Felix Schumacher) (50467570)',
    'GTA Analytics (Fabian Birzele) (50606589)',
    'GTAE Computational CoE (Jörg Degen) (50682980)',
    'GSFHB Computational Catalyst (Jens Reeder) (30931809)',
    'GSAA Prescient AI ML (Vladimir Gligorijevic) (50364410)',
    'GSAG Prescient Frontier Research (Stephen Ra) (50427018)',
    'MMED Data Backbone (Dominik Wendel) (50407023)',
    'PRDF Lead Discovery (Federica Morandi) (50225496)',
    'POR Discovery Oncology (Ashley Lakner) (25696339)',
    'TNDAB Neurodegeneration (Christopher Lane) (50663284)',
    'PCE Early Development (Luka Kulic) (50310171)'
]

df_slide = df[df['supervisory_organization'].isin(orgs)]

# >>> df_slide
#             job_id                                              title company_name                                        description  ...                    job_level grade                         job_family  is_evergreen
# 0    202109-125486                                    Software Tester        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor  None                        Unspecified             1
# 1    202109-125765                                           Laborant        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor  None                        Unspecified             1
# 2    202203-110282                  system development troubleshooter        Roche  <div><p><b>Who We Are</b></p><p></p><p>Roche D...  ...       Individual Contributor  None             Production Engineering             1
# 4    202302-103172                              Entwicklungsingenieur        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor  None             Production Engineering             1
# 5    202402-104044                              Entwicklungsingenieur        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor  None             Production Engineering             1
# 19   202505-113105  group leader, high throughput screening and pr...        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL8                           Research             0
# 30   202506-115574       lead clinical director - alzheimer's disease        Roche  <p style="text-align:left"><span>At Roche you ...  ...       Individual Contributor   SE7               Clinical Development             0
# 33   202506-115770                                 Veterinärpathologe        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE6                           Research             0
# 50   202507-117136  oncology discovery research unit head - target...        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL9                           Research             0
# 61   202507-117766                      head of computational biology        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL9                           Research             0
# 63   202507-117910                                       Group Leader        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL8                           Research             0
# 79   202507-118937                            Leiter Daten Governance        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL9                         General IT             0
# 80   202507-118940  head of pt business it ot architecture & stand...        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL8                         General IT             0
# 81   202507-118944                         Leiter Datenbeschleunigung        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL9                         General IT             0
# 82   202507-118946                            Leiter Datenengineering        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL8                         General IT             0
# 83   202507-118960                                               Head        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...  Manager with direct reports   PL8                         General IT             0
# 89   202508-119792     scientist , synthetic biology genomic medicine        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE6                           Research             0
# 90   202508-119794  research associate , synthetic biology genomic...        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE5                           Research             0
# 101  202508-120565  senior scientific software engineer , analytic...        Roche  <h3>The Position</h3><p><span>A healthier futu...  ...       Individual Contributor   SE6      Devices / Systems / Solutions             0
# 103  202508-120616                                        RNA-Biologe        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE5                           Research             0
# 104  202508-120619                                        RNA-Biologe        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE6                           Research             0
# 109  202508-120731                 Senior Scientist, Machine Learning        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE6  Design Engineering & Architecture             0
# 111  202508-120775              Medical Director — Multiple Sclerosis        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE8               Clinical Development             0
# 123  202508-121386         siRNA Chemistry & Drug Discovery Scientist        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE6                           Research             0
# 126  202508-121514  Leitender Wissenschaftler für maschinelles Lernen        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE7  Design Engineering & Architecture             0
# 148  202509-122166                     research associate in genomics        Roche  <p>Bei Roche kannst du ganz du selbst sein und...  ...       Individual Contributor   SE5                           Research             0
#
# [26 rows x 20 columns]
