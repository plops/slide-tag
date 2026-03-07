use crate::models::Job;
use libsql::{params, Connection};

pub struct JobRepository {
    conn: Connection,
}

impl JobRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub async fn insert_job(&self, job: &Job) -> anyhow::Result<i64> {
        self.conn
            .execute(
                "INSERT INTO jobs (title, description, location) VALUES (?1, ?2, ?3)",
                params![
                    job.title.clone(),
                    job.description.as_deref(),
                    job.location.clone()
                ],
            )
            .await?;
        Ok(self.conn.last_insert_rowid())
    }

    pub async fn get_all_jobs(&self) -> anyhow::Result<Vec<Job>> {
        let mut rows = self
            .conn
            .query("SELECT id, title, description, location FROM jobs", ())
            .await?;
        let mut jobs = Vec::new();
        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let description: Option<String> = row.get(2)?;
            let location: String = row.get(3)?;
            jobs.push(Job {
                id: Some(id),
                title,
                description,
                location,
            });
        }
        Ok(jobs)
    }
}
