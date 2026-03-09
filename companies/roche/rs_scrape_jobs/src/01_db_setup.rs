use libsql::Builder;

pub async fn init_db(db_path: &str) -> anyhow::Result<libsql::Connection> {
    let db = Builder::new_local(db_path).build().await?;
    let conn = db.connect()?;

    // Create jobs table if it doesn't exist (preserve existing data)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            identifier TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            location TEXT,
            organization TEXT,
            required_topics TEXT,
            nice_to_haves TEXT,
            pay_grade TEXT,
            sub_category TEXT,
            category_raw TEXT,
            employment_type TEXT,
            work_hours TEXT,
            worker_type TEXT,
            job_profile TEXT,
            supervisory_organization TEXT,
            target_hire_date TEXT,
            no_of_available_openings TEXT,
            grade_profile TEXT,
            recruiting_start_date TEXT,
            job_level TEXT,
            job_family TEXT,
            job_type TEXT,
            is_evergreen TEXT,
            standardised_country TEXT,
            run_date TEXT,
            run_id TEXT,
            address_locality TEXT,
            address_region TEXT,
            address_country TEXT,
            postal_code TEXT,
            job_summary TEXT,
            slide_tag_relevance INTEGER
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS skills (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_skills (
            job_id INTEGER,
            skill_id INTEGER
        )",
        (),
    )
    .await?;

    // New tables for stage 7
    conn.execute(
        "CREATE TABLE IF NOT EXISTS candidates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            oauth_sub TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            profile_text TEXT
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS candidate_matches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            candidate_id INTEGER NOT NULL,
            job_identifier TEXT NOT NULL,
            model_used TEXT NOT NULL,
            score REAL NOT NULL,
            explanation TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            identifier TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            location TEXT,
            organization TEXT,
            required_topics TEXT,
            nice_to_haves TEXT,
            pay_grade TEXT,
            sub_category TEXT,
            category_raw TEXT,
            employment_type TEXT,
            work_hours TEXT,
            worker_type TEXT,
            job_profile TEXT,
            supervisory_organization TEXT,
            target_hire_date TEXT,
            no_of_available_openings TEXT,
            grade_profile TEXT,
            recruiting_start_date TEXT,
            job_level TEXT,
            job_family TEXT,
            job_type TEXT,
            is_evergreen TEXT,
            standardised_country TEXT,
            run_date TEXT,
            run_id TEXT,
            address_locality TEXT,
            address_region TEXT,
            address_country TEXT,
            postal_code TEXT,
            job_summary TEXT,
            slide_tag_relevance INTEGER,
            created_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    // Create indexes for better query performance
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_candidate_id ON candidate_matches(candidate_id)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_score ON candidate_matches(score DESC)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_job_hist_identifier ON job_history(identifier)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_job_hist_title ON job_history(title)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_job_hist_description ON job_history(description)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_job_hist_created_at ON job_history(created_at DESC)",
        (),
    )
    .await?;

    Ok(conn)
}
