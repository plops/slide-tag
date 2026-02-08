import sqlite_minutils
import grpc
from google.ai.generativelanguage.v1beta import (
    generative_service_pb2_grpc as gen_service,
    generative_service_pb2 as gen_types,
    content_pb2 as content_types
)
# uv add google-api-python-client grpcio grpcio-tools pydantic
#export GDIR=~/src/googleapis; uv run python -m grpc_tools.protoc -I $GDIR --python_out=. --grpc_python_out=. $GDIR/google/ai/generativelanguage/v1beta/*.proto

db = sqlite_minutils.Database("novartis_jobs.db")
for row in db["jobs"].rows:
    print(row["job_id"], row["title"])

# print as pandas DataFrame
import pandas as pd
df = pd.DataFrame(db["jobs"].rows)
print(df.head())
print(df.iloc[1])

# REQ-10068139 Senior Expert Engineering – Assembly for Medical Device
# REQ-10069219 Market Access Manager (80-100%*) (temp role 12 months)
# REQ-10068872 Global Program Safety Lead

#          job_id                                              title                                                url                                        description  ...   posted_date                                          apply_url                                       html_content                  scraped_at
# 0  REQ-10068139  Senior Expert Engineering – Assembly for Medic...  https://www.novartis.com/careers/career-search...  Location: Basel, Switzerland #onsite\n\nRole P...  ...  Dec 11, 2025  https://novartis.wd3.myworkdayjobs.com/en-US/N...  <!DOCTYPE html><html lang="en" dir="ltr" prefi...  2026-01-19T18:17:45.114457
# 1  REQ-10069219  Market Access Manager (80-100%*) (temp role 12...  https://www.novartis.com/careers/career-search...  Возглавляет реализацию стратегии устойчивого д...  ...  Jan 13, 2026  https://novartis.wd3.myworkdayjobs.com/ru-RU/N...  <!DOCTYPE html><html lang="en" dir="ltr" prefi...  2026-01-19T18:18:14.516673
# 2  REQ-10068872                         Global Program Safety Lead  https://www.novartis.com/careers/career-search...  Are you ready to make a significant impact in ...  ...  Jan 12, 2026  https://novartis.wd3.myworkdayjobs.com/en-US/N...  <!DOCTYPE html><html lang="en" dir="ltr" prefi...  2026-01-19T18:18:10.613458
# 3  REQ-10064204                 Senior Patent Litigation Paralegal  https://www.novartis.com/careers/career-search...  Join a dynamic team where you will be managing...  ...  Nov 21, 2025  https://novartis.wd3.myworkdayjobs.com/en-US/N...  <!DOCTYPE html><html lang="en" dir="ltr" prefi...  2026-01-19T18:17:26.603847
# 4  REQ-10067743  Senior Expert Radioligand Parenteral Packaging...  https://www.novartis.com/careers/career-search...  #LI-Hybrid\n\nLocation: Schaftenau, Austria\n\...  ...  Jan 08, 2026  https://novartis.wd3.myworkdayjobs.com/en-US/N...  <!DOCTYPE html><html lang="en" dir="ltr" prefi...  2026-01-19T18:17:22.306087
#
# [5 rows x 13 columns]
# job_id                                                REQ-10069219
# title            Market Access Manager (80-100%*) (temp role 12...
# url              https://www.novartis.com/careers/career-search...
# description      Возглавляет реализацию стратегии устойчивого д...
# division                                             International
# business_unit                            Strategic Planning & BD&L
# site                                       Rotkreuz (Office-Based)
# location                                               Switzerland
# job_type                                                 Full time
# posted_date                                           Jan 13, 2026
# apply_url        https://novartis.wd3.myworkdayjobs.com/ru-RU/N...
# html_content     <!DOCTYPE html><html lang="en" dir="ltr" prefi...
# scraped_at                              2026-01-19T18:18:14.516673
# Name: 1, dtype: object

prompt = f"""Analyze the job descriptions below in the context of Slide-tag and related spatial 
genomics technologies. These technologies integrate techniques like Next-Generation Sequencing (NGS), single-cell/nucleus RNA 
sequencing (sc/snRNA-seq), molecular pathology, and complex bioinformatics to map gene activity in tissue.
The output should be a JSON object with a list containing three fields for each job description:
1. `job_summary`: A bullet-point summary (as an array of strings) of the key responsibilities and required qualifications.
2. `slide_tag_relevance`: An integer score from 1 (unrelated) to 5 (highly relevant), rating the job's connection to the development or application of these technologies.
3. `idx`: The index of the job in the input list (for tracking purposes).

{job_description}"""