use crate::db_traits::DatabaseProvider;
use crate::models::{Candidate, CandidateMatch, Job, Location, Skill};
use chrono::{DateTime, Utc};
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
                "INSERT INTO jobs (identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, employment_type, work_hours, worker_type, job_profile, supervisory_organization, target_hire_date, no_of_available_openings, grade_profile, recruiting_start_date, job_level, job_family, job_type, is_evergreen, standardised_country, run_date, run_id, address_locality, address_region, address_country, postal_code, job_summary) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31)",
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
                    job.postal_code.as_deref(),
                    job.job_summary.as_deref()
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn update_job_ai(&self, identifier: &str, summary: &str) -> anyhow::Result<()> {
        self.conn
            .execute(
                "UPDATE jobs SET job_summary = ? WHERE identifier = ?",
                params![summary, identifier],
            )
            .await?;
        Ok(())
    }

    pub async fn get_unannotated_jobs(&self, limit: usize) -> anyhow::Result<Vec<Job>> {
        let mut rows = self
            .conn
            .query(
                "SELECT identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, employment_type, work_hours, worker_type, job_profile, supervisory_organization, target_hire_date, no_of_available_openings, grade_profile, recruiting_start_date, job_level, job_family, job_type, is_evergreen, standardised_country, run_date, run_id, address_locality, address_region, address_country, postal_code, job_summary FROM jobs WHERE job_summary IS NULL LIMIT ?",
                params![limit as i64],
            )
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
            let job_summary: Option<String> = row.get(30)?;
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
                job_summary,
            });
        }
        Ok(jobs)
    }

    pub async fn get_all_jobs(&self) -> anyhow::Result<Vec<Job>> {
        let mut rows = self
            .conn
            .query("SELECT identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, employment_type, work_hours, worker_type, job_profile, supervisory_organization, target_hire_date, no_of_available_openings, grade_profile, recruiting_start_date, job_level, job_family, job_type, is_evergreen, standardised_country, run_date, run_id, address_locality, address_region, address_country, postal_code, job_summary FROM jobs", ())
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
            let job_summary: Option<String> = row.get(30)?;
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
                job_summary,
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

    pub async fn get_candidate_count(&self) -> anyhow::Result<i64> {
        let mut rows = self
            .conn
            .query("SELECT COUNT(*) FROM candidates", ())
            .await?;
        if let Some(row) = rows.next().await? {
            let cnt: i64 = row.get(0)?;
            Ok(cnt)
        } else {
            Ok(0)
        }
    }
}

#[async_trait::async_trait]
impl DatabaseProvider for JobRepository {
    async fn insert_job_history(&self, job: &Job) -> anyhow::Result<()> {
        let created_at = Utc::now();
        self.conn
            .execute(
                "INSERT INTO job_history (identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, employment_type, work_hours, worker_type, job_profile, supervisory_organization, target_hire_date, no_of_available_openings, grade_profile, recruiting_start_date, job_level, job_family, job_type, is_evergreen, standardised_country, run_date, run_id, address_locality, address_region, address_country, postal_code, job_summary, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
                    job.postal_code.as_deref(),
                    job.job_summary.as_deref(),
                    created_at.to_rfc3339()
                ],
            )
            .await?;
        Ok(())
    }

    async fn get_latest_jobs(&self) -> anyhow::Result<Vec<Job>> {
        let sql = "SELECT jh.identifier, jh.title, jh.description, jh.location, jh.organization, jh.required_topics, jh.nice_to_haves, jh.pay_grade, jh.sub_category, jh.category_raw, jh.employment_type, jh.work_hours, jh.worker_type, jh.job_profile, jh.supervisory_organization, jh.target_hire_date, jh.no_of_available_openings, jh.grade_profile, jh.recruiting_start_date, jh.job_level, jh.job_family, jh.job_type, jh.is_evergreen, jh.standardised_country, jh.run_date, jh.run_id, jh.address_locality, jh.address_region, jh.address_country, jh.postal_code, jh.job_summary FROM job_history jh INNER JOIN (SELECT identifier, MAX(id) as max_id FROM job_history GROUP BY identifier) latest ON jh.identifier = latest.identifier AND jh.id = latest.max_id";
        let mut rows = self.conn.query(sql, ()).await?;
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
            let job_summary: Option<String> = row.get(30)?;
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
                job_summary,
            });
        }
        Ok(jobs)
    }

    async fn upsert_candidate(&self, candidate: &Candidate) -> anyhow::Result<i64> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO candidates (oauth_sub, name, profile_text) VALUES (?, ?, ?)",
                params![candidate.oauth_sub.clone(), candidate.name.clone(), candidate.profile_text.clone()],
            )
            .await?;
        Ok(self.conn.last_insert_rowid())
    }

    async fn insert_candidate_match(&self, match_data: &CandidateMatch) -> anyhow::Result<()> {
        self.conn
            .execute(
                "INSERT INTO candidate_matches (candidate_id, job_identifier, model_used, score, explanation, created_at) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    match_data.candidate_id,
                    match_data.job_identifier.clone(),
                    match_data.model_used.clone(),
                    match_data.score,
                    match_data.explanation.clone(),
                    match_data.created_at.to_rfc3339()
                ],
            )
            .await?;
        Ok(())
    }

    async fn get_matches_for_candidate(
        &self,
        candidate_id: i64,
    ) -> anyhow::Result<Vec<CandidateMatch>> {
        let mut rows = self.conn
            .query(
                "SELECT id, candidate_id, job_identifier, model_used, score, explanation, created_at FROM candidate_matches WHERE candidate_id = ? ORDER BY created_at DESC",
                params![candidate_id],
            )
            .await?;
        let mut matches = Vec::new();
        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let candidate_id: i64 = row.get(1)?;
            let job_identifier: String = row.get(2)?;
            let model_used: String = row.get(3)?;
            let score_f64: f64 = row.get(4)?;
            let score = score_f64 as f32;
            let explanation: String = row.get(5)?;
            let created_at_str: String = row.get(6)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
            matches.push(CandidateMatch {
                id: Some(id),
                candidate_id,
                job_identifier,
                model_used,
                score,
                explanation,
                created_at,
            });
        }
        Ok(matches)
    }
}
