use crate::models::{Job, Location, Skill};
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

    pub async fn insert_skill(&self, skill: &Skill) -> anyhow::Result<i64> {
        self.conn
            .execute(
                "INSERT INTO skills (name) VALUES (?)",
                params![skill.name.clone()],
            )
            .await?;
        Ok(self.conn.last_insert_rowid())
    }

    pub async fn get_all_skills(&self) -> anyhow::Result<Vec<Skill>> {
        let mut rows = self.conn.query("SELECT id, name FROM skills", ()).await?;
        let mut skills = Vec::new();
        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            skills.push(Skill { id: Some(id), name });
        }
        Ok(skills)
    }

    pub async fn insert_location(&self, location: &Location) -> anyhow::Result<i64> {
        self.conn
            .execute(
                "INSERT INTO locations (name) VALUES (?)",
                params![location.name.clone()],
            )
            .await?;
        Ok(self.conn.last_insert_rowid())
    }

    pub async fn get_all_locations(&self) -> anyhow::Result<Vec<Location>> {
        let mut rows = self
            .conn
            .query("SELECT id, name FROM locations", ())
            .await?;
        let mut locations = Vec::new();
        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            locations.push(Location { id: Some(id), name });
        }
        Ok(locations)
    }

    pub async fn insert_job_skill(&self, job_id: i64, skill_id: i64) -> anyhow::Result<()> {
        self.conn
            .execute(
                "INSERT INTO job_skills (job_id, skill_id) VALUES (?, ?)",
                params![job_id, skill_id],
            )
            .await?;
        Ok(())
    }
}
