use libsql::params;
use rs_scrape::db_setup::init_db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Debugging database insertion...");

    // Create fresh database
    let conn = init_db("debug_insert.db").await?;

    // Check actual table schema
    println!("Checking table schema...");
    let mut rows = conn.query("PRAGMA table_info(job_history)", ()).await?;
    let mut column_count = 0;
    while let Some(row) = rows.next().await? {
        let name: String = row.get(1)?;
        println!("Column {}: {}", column_count, name);
        column_count += 1;
    }
    println!("Total columns: {}", column_count);

    // Try with fewer parameters first
    println!("Trying INSERT with just required columns...");
    let result = conn.execute(
        "INSERT INTO job_history (identifier, title, description, created_at) VALUES (?, ?, ?, ?)",
        params![
            "test_job_simple",
            "Test Engineer", 
            "Test description",
            "2023-01-01T00:00:00Z"
        ],
    ).await;

    match result {
        Ok(_) => println!("✅ Simple INSERT successful!"),
        Err(e) => println!("❌ Simple INSERT failed: {}", e),
    }

    // Try with more columns to see where it breaks
    println!("Trying INSERT with 10 columns...");
    let result = conn.execute(
        "INSERT INTO job_history (identifier, title, description, location, organization, required_topics, nice_to_haves, pay_grade, sub_category, category_raw, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            "test_job_10",
            "Test Engineer", 
            "Test description",
            "Test Location",
            "Test Org",
            None as Option<String>, // required_topics
            None as Option<String>, // nice_to_haves
            None as Option<&str>,   // pay_grade
            None as Option<&str>,   // sub_category
            None as Option<&str>,   // category_raw
            "2023-01-01T00:00:00Z"
        ],
    ).await;

    match result {
        Ok(_) => println!("✅ 10-column INSERT successful!"),
        Err(e) => println!("❌ 10-column INSERT failed: {}", e),
    }

    Ok(())
}
