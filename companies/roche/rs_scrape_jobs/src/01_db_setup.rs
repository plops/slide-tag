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
            slide_tag_relevance TEXT
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

    Ok(conn)
}
