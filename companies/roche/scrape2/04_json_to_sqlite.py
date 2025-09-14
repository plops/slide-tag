import json
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
    'worker_type': str,
    'job_level': str,
    'job_family': str
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


# --- 2. Data Parsing and Insertion ---

# Load the JSON data from the file
with open('job_description.json', 'r') as f:
    data = json.load(f)

job_data = data['jobDetail']['data']['job']

# Extract job information
job_id = job_data['reqId']
title = job_data['ml_title']
company_name = job_data['companyName']
description = job_data['structureData']['description']
date_posted = job_data['datePosted'].split('T')[0]
apply_url = job_data['applyUrl']
# Extract salary from description string and handle potential errors
salary_range_str = job_data['description'].split('The expected salary range for this position based on the primary location of South San Francisco, CA is ')[1].split(' USD Annual.')[0]
salary_parts = salary_range_str.replace('$', '').replace(',', '').split(' - ')
salary_min = int(float(salary_parts[0]))
salary_max = int(float(salary_parts[1]))
worker_type = job_data['additionalFields']['workerType']
job_level = job_data['jobLevel']
job_family = job_data['jobFamily']

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
    'worker_type': worker_type,
    'job_level': job_level,
    'job_family': job_family
}, ignore=True)


# Populate Locations and Job_Locations tables
for location in job_data['standardised_multi_location']:
    loc_record = {
        'city': location.get('standardisedCity'),
        'state': location.get('standardisedState'),
        'country': location.get('standardisedCountry'),
        'postal_code': job_data['multi_location'][0].get('postalCode'),
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
skills_list = [{'skill_name': skill} for skill in job_data['ml_skills']]
# Use upsert_all for efficient bulk insertion/updates
skills_table.upsert_all(skills_list, pk='skill_name')

for skill in job_data['ml_skills']:
    # Get the ID of the skill
    skill_record = next(skills_table.rows_where('skill_name = ?', [skill]))
    skill_id = skill_record['skill_id']

    # Link the job to the skill
    job_skills_table.insert({'job_id': job_id, 'skill_id': skill_id}, ignore=True)


# Populate Education_Requirements table
education_info = job_data['ml_job_parser']['education']
if education_info:
    degree_level = "PhD"
    education_records = [{
        'job_id': job_id,
        'degree_level': degree_level,
        'field_of_study': field
    } for field in education_info[0].get('fieldOfStudy', [])]
    education_table.insert_all(education_records)

print("Database 'jobs_minutils.db' has been successfully created and populated.")
print("\nDatabase Schema:")
print(db.schema)