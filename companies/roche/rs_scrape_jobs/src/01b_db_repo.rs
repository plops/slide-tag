use crate::models::{Job, Location, Skill};
use libsql::{params, Connection};

pub struct JobRepository {
    conn: Connection,
}

impl JobRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub async fn insert_job(&self, job: &Job) -> anyhow::Result<()> {
        self.conn
            .execute(
                "INSERT INTO jobs (identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, employment_type, work_hours, worker_type, job_profile, supervisory_organization, target_hire_date, no_of_available_openings, grade_profile, recruiting_start_date, job_level, job_family, job_type, is_evergreen, standardised_country, run_date, run_id, address_locality, address_region, address_country, postal_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)",
                params![
                    job.identifier.clone(),
                    job.title.clone(),
                    job.description.as_deref(),
                    job.location.clone(),
                    job.organization.as_deref(),
                    job.required_topics.as_ref().map(serde_json::to_string).transpose()?,
                    job.nice_to_haves.as_ref().map(serde_json::to_string).transpose()?,
                    job.pay_grade.as_deref(),
                    job.sub_category.as_deref(),
                    job.category_raw.as_deref(),
                    job.employment_type.as_deref(),
                    job.work_hours.as_deref(),
                    job.worker_type.as_deref(),
                    job.job_profile.as_deref(),
                    job.supervisory_organization.as_deref(),
                    job.target_hire_date.as_deref(),
                    job.no_of_available_openings.as_deref(),
                    job.grade_profile.as_deref(),
                    job.recruiting_start_date.as_deref(),
                    job.job_level.as_deref(),
                    job.job_family.as_deref(),
                    job.job_type.as_deref(),
                    job.is_evergreen.as_deref(),
                    job.standardised_country.as_deref(),
                    job.run_date.as_deref(),
                    job.run_id.as_deref(),
                    job.address_locality.as_deref(),
                    job.address_region.as_deref(),
                    job.address_country.as_deref(),
                    job.postal_code.as_deref()
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_all_jobs(&self) -> anyhow::Result<Vec<Job>> {
        let mut rows = self
            .conn
            .query("SELECT identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, employment_type, work_hours, worker_type, job_profile, supervisory_organization, target_hire_date, no_of_available_openings, grade_profile, recruiting_start_date, job_level, job_family, job_type, is_evergreen, standardised_country, run_date, run_id, address_locality, address_region, address_country, postal_code FROM jobs", ())
            .await?;
        let mut jobs = Vec::new();
        while let Some(row) = rows.next().await? {
            let identifier: String = row.get(0)?;
            let title: String = row.get(1)?;
            let description: Option<String> = row.get(2)?;
            let location: String = row.get(3)?;
            let organization: Option<String> = row.get(4)?;
            let required_topics: Option<String> = row.get(5)?;
            let nice_to_haves: Option<String> = row.get(6)?;
            let pay_grade: Option<String> = row.get(7)?;
            let sub_category: Option<String> = row.get(8)?;
            let category_raw: Option<String> = row.get(9)?;
            let employment_type: Option<String> = row.get(10)?;
            let work_hours: Option<String> = row.get(11)?;
            let worker_type: Option<String> = row.get(12)?;
            let job_profile: Option<String> = row.get(13)?;
            let supervisory_organization: Option<String> = row.get(14)?;
            let target_hire_date: Option<String> = row.get(15)?;
            let no_of_available_openings: Option<String> = row.get(16)?;
            let grade_profile: Option<String> = row.get(17)?;
            let recruiting_start_date: Option<String> = row.get(18)?;
            let job_level: Option<String> = row.get(19)?;
            let job_family: Option<String> = row.get(20)?;
            let job_type: Option<String> = row.get(21)?;
            let is_evergreen: Option<String> = row.get(22)?;
            let standardised_country: Option<String> = row.get(23)?;
            let run_date: Option<String> = row.get(24)?;
            let run_id: Option<String> = row.get(25)?;
            let address_locality: Option<String> = row.get(26)?;
            let address_region: Option<String> = row.get(27)?;
            let address_country: Option<String> = row.get(28)?;
            let postal_code: Option<String> = row.get(29)?;
            let required_topics_parsed = required_topics
                .as_ref()
                .map(|s| serde_json::from_str(s))
                .transpose()
                .ok()
                .flatten();
            let nice_to_haves_parsed = nice_to_haves
                .as_ref()
                .map(|s| serde_json::from_str(s))
                .transpose()
                .ok()
                .flatten();
            jobs.push(Job {
                identifier,
                title,
                description,
                location,
                organization,
                required_topics: required_topics_parsed,
                nice_to_haves: nice_to_haves_parsed,
                pay_grade,
                sub_category,
                category_raw,
                employment_type,
                work_hours,
                worker_type,
                job_profile,
                supervisory_organization,
                target_hire_date,
                no_of_available_openings,
                grade_profile,
                recruiting_start_date,
                job_level,
                job_family,
                job_type,
                is_evergreen,
                standardised_country,
                run_date,
                run_id,
                address_locality,
                address_region,
                address_country,
                postal_code,
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
