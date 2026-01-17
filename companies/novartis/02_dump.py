import sqlite_minutils

db = sqlite_minutils.Database("novartis_jobs.db")
for row in db["jobs"].rows:
    print(row["job_id"], row["title"])

# print as pandas DataFrame
import pandas as pd
df = pd.DataFrame(db["jobs"].rows)
print(df.head())
