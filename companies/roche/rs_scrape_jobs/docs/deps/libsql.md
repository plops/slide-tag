# LibSQL (Local SQLite) Documentation & Examples

The `libsql` Rust client (the core behind Turso) is used here primarily for high-performance, asynchronous access to **local SQLite database files**. It is preferred over standard `rusqlite` for its native `tokio` support and modern API.

## Core Features
- **Local SQLite**: Extremely fast reads and writes to a standard `.db` file.
- **Asynchronous**: Fully compatible with `tokio`, preventing blocking I/O on the main executor.
- **SQLite Format**: Uses standard SQLite files, meaning they can be opened by any standard viewer (DB Browser, etc.).

## Examples

### Primary Use Case: Local File Connection
This is how the scraper will store and retrieve job data.

```rust
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create or open the local database file "jobs_minutils.db"
    let db = Builder::new_local("jobs_minutils.db").build().await?;
    let conn = db.connect()?;
    
    // Initialize schema
    conn.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY, 
            title TEXT, 
            description TEXT,
            relevance_score INTEGER,
            added_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )", 
        ()
    ).await?;
    
    Ok(())
}
```

### Running Queries
The API is designed for ergonomic async usage.

```rust
// Querying results
let mut rows = conn.query("SELECT title FROM jobs WHERE relevance_score > ?1", params![80]).await?;
while let Some(row) = rows.next().await? {
    let title: String = row.get(0)?;
    println!("High Score Job: {}", title);
}

// Optimized insertions via prepared statements
let stmt = conn.prepare("INSERT INTO jobs (id, title) VALUES (?1, ?2) ON CONFLICT(id) DO NOTHING").await?;
stmt.execute(params!["0001", "Software Engineer"]).await?;
```

## Relevant Tasks in This Project
- **Local Storage**: Implementing the port of `04_json_to_sqlite.py` using `libsql`.
- **Async Efficiency**: Ensuring the scraper (Phase 1) can write to the DB without stalling browser automation.
- **Schema Management**: Handling the job database structure locally.

Note: While `libsql` supports remote syncing to Turso, that feature is **not required** for the core scraper logic and is out of scope for the current local-first architecture.
