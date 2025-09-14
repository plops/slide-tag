import json
import argparse
import sys
from sqlite_minutils.db import *

# --- 1. Database and Table Setup ---

# Initialize the database. This creates 'jobs_minutils.db' if it doesn't exist.
db = Database('jobs_minutils.db')

# Instantiate Table objects
jobs_table = Table(db, 'Jobs')
locations_table = Table(db, 'Locations')
job_locations_table = Table(db, 'Job_Locations')
skills_table = Table(db, 'Skills')
job_skills_table = Table(db, 'Job_Skills')
education_table = Table(db, 'Education_Requirements')

# Drop existing tables for a clean run
for t in db.tables:
    t.drop()

# Create tables with specified schema, primary keys, and foreign keys
jobs_table.create({
    'job_id': str,
    'title': str,
    'company_name': str,
    'description': str,
    'date_posted': str, # Stored as TEXT
    'apply_url': str,
    'salary_min': int,
    'salary_max': int,
    # --- MODIFIED: Added all new fields from additionalFields ---
    'worker_type': str,
    'sub_category': str,
    'job_profile': str,
    'supervisory_organization': str,
    'target_hire_date': str, # Stored as TEXT
    'openings': int,
    'grade_profile': str,
    'recruiting_start_date': str, # Stored as TEXT
    'job_level': str,
    'grade': str,
    'job_family': str,
    'is_evergreen': int # Stored as integer (0 or 1)
}, pk='job_id')

locations_table.create({
    'location_id': int,
    'city': str,
    'state': str,
    'country': str,
    'postal_code': str,
    'latitude': float,
    'longitude': float
}, pk='location_id')

skills_table.create({
    'skill_id': int,
    'skill_name': str
}, pk='skill_id')

education_table.create({
    'education_id': int,
    'job_id': str,
    'degree_level': str,
    'field_of_study': str
}, pk='education_id', foreign_keys=[('job_id', 'Jobs', 'job_id')])


# Create junction tables with composite primary keys and foreign keys
job_locations_table.create({
    'job_id': str,
    'location_id': int
}, pk=('job_id', 'location_id'), foreign_keys=[
    ('job_id', 'Jobs', 'job_id'),
    ('location_id', 'Locations', 'location_id')
])

job_skills_table.create({
    'job_id': str,
    'skill_id': int
}, pk=('job_id', 'skill_id'), foreign_keys=[
    ('job_id', 'Jobs', 'job_id'),
    ('skill_id', 'Skills', 'skill_id')
])


# --- 2. Extraction & Insertion Logic (refactored into a function) ---

def process_job(job_data):
    # Extract job information (robust to missing fields)
    job_id = job_data.get('reqId') or job_data.get('id')
    title = job_data.get('ml_title')
    company_name = job_data.get('companyName')
    description = job_data.get('structureData', {}).get('description') or job_data.get('description')
    date_posted = job_data.get('datePosted', '').split('T')[0] if job_data.get('datePosted') else None
    apply_url = job_data.get('applyUrl')

    # Extract salary robustly
    salary_min = None
    salary_max = None
    try:
        # original code assumes a specific phrase — try that first
        if job_data.get('description') and 'The expected salary range for this position' in job_data['description']:
            salary_range_str = job_data['description'].split('The expected salary range for this position based on the primary location of South San Francisco, CA is ')[1].split(' USD Annual.')[0]
            salary_parts = salary_range_str.replace('$', '').replace(',', '').split(' - ')
            salary_min = int(float(salary_parts[0]))
            salary_max = int(float(salary_parts[1]))
    except Exception:
        # best-effort fallback: leave as None
        salary_min = None
        salary_max = None

    # --- MODIFIED: Extract all fields from the 'additionalFields' sub-dictionary ---
    additional_fields = job_data.get('additionalFields', {})
    worker_type = additional_fields.get('workerType')
    sub_category = additional_fields.get('subCategory')
    job_profile = additional_fields.get('jobProfile')
    supervisory_organization = additional_fields.get('supervisoryOrganization')
    target_hire_date = additional_fields.get('targetHireDate', '').split('T')[0] if additional_fields.get('targetHireDate') else None
    openings = additional_fields.get('noOfAvailableOpenings')
    grade_profile = additional_fields.get('gradeProfile')
    recruiting_start_date = additional_fields.get('recruitingStartDate', '').split('T')[0] if additional_fields.get('recruitingStartDate') else None
    job_level = additional_fields.get('jobLevel')
    grade = additional_fields.get('grade')
    job_family = additional_fields.get('jobFamily')
    is_evergreen = additional_fields.get('isEvergreen')


    # Insert the main job record
    jobs_table.insert({
        'job_id': job_id,
        'title': title,
        'company_name': company_name,
        'description': description,
        'date_posted': date_posted,
        'apply_url': apply_url,
        'salary_min': salary_min,
        'salary_max': salary_max,
        # --- MODIFIED: Add new fields to the insert statement ---
        'worker_type': worker_type,
        'sub_category': sub_category,
        'job_profile': job_profile,
        'supervisory_organization': supervisory_organization,
        'target_hire_date': target_hire_date,
        'openings': openings,
        'grade_profile': grade_profile,
        'recruiting_start_date': recruiting_start_date,
        'job_level': job_level,
        'grade': grade,
        'job_family': job_family,
        'is_evergreen': is_evergreen
    }, ignore=True)

    # Populate Locations and Job_Locations tables
    for location in job_data.get('standardised_multi_location', []):
        loc_record = {
            'city': location.get('standardisedCity'),
            'state': location.get('standardisedState'),
            'country': location.get('standardisedCountry'),
            'postal_code': job_data.get('multi_location', [{}])[0].get('postalCode'),
            'latitude': location.get('latitude'),
            'longitude': location.get('longitude')
        }
        # Use upsert to avoid duplicates based on a unique key
        locations_table.upsert(loc_record, pk=['city', 'state', 'country'])

        # Get the ID of the location we just inserted/found
        inserted_loc = next(locations_table.rows_where(
            'city = :city AND state = :state AND country = :country',
            loc_record
        ))
        location_id = inserted_loc['location_id']

        # Link the job to the location
        job_locations_table.insert({
            'job_id': job_id,
            'location_id': location_id
        }, ignore=True)


    # Populate Skills and Job_Skills tables
    for skill in job_data.get('ml_skills', []):
        # upsert skills by name to avoid duplicates
        skills_table.upsert({'skill_name': skill}, pk='skill_name')

    for skill in job_data.get('ml_skills', []):
        # Get the ID of the skill
        skill_record = next(skills_table.rows_where('skill_name = ?', [skill]))
        skill_id = skill_record['skill_id']

        # Link the job to the skill
        job_skills_table.insert({'job_id': job_id, 'skill_id': skill_id}, ignore=True)


    # Populate Education_Requirements table
    education_info = job_data.get('ml_job_parser', {}).get('education', [])
    if education_info:
        # Use entries inside the first education item if present
        education_records = []
        degree_level = education_info[0].get('degreeLevel') or "PhD"
        for field in education_info[0].get('fieldOfStudy', []):
            education_records.append({
                'job_id': job_id,
                'degree_level': degree_level,
                'field_of_study': field
            })
        if education_records:
            education_table.insert_all(education_records)


# --- 3. CLI handling to process multiple JSON files ---
def main(argv):
    parser = argparse.ArgumentParser(description="Insert one or more job JSON files into jobs_minutils.db")
    parser.add_argument('files', nargs='+', help='One or more JSON files to process')
    args = parser.parse_args(argv)

    processed = 0
    for fp in args.files:
        print(f"Processing file: {fp}")
        try:
            with open(fp, 'r') as f:
                data = json.load(f)
            # support both nested path and direct job object
            job_data = None
            if isinstance(data, dict):
                # original layout: data['jobDetail']['data']['job']
                job_data = (data.get('jobDetail', {}) .get('data', {}) .get('job')) if data.get('jobDetail') else data.get('job') or data.get('job_data') or data
            if not job_data:
                print(f"  Warning: no job data found in {fp}, skipping")
                continue

            process_job(job_data)
            processed += 1
            print(f"  Inserted job {job_data.get('reqId') or job_data.get('id')}")
        except Exception as e:
            print(f"  Error processing {fp}: {e}")

    print(f"\nProcessed {processed} file(s). Database 'jobs_minutils.db' has been updated.")
    print("\nDatabase Schema:")
    print(db.schema)


if __name__ == '__main__':
    main(sys.argv[1:])