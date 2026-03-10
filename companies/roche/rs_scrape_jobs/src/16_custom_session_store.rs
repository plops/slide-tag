//! Custom libsql session store implementation
//!
//! This provides a persistent session store using our existing libsql database
//! with tower-sessions 0.14.0 compatibility.

use crate::db_traits::DatabaseProvider;
use anyhow::Result;
use async_trait::async_trait;
use libsql::params;
use std::collections::HashMap;
use std::sync::Arc;
use time::OffsetDateTime;
use tower_sessions_core::{
    session::{Id, Record},
    session_store::{Error, ExpiredDeletion, SessionStore},
};

/// Custom session store using libsql database
#[derive(Clone)]
pub struct LibsqlSessionStore {
    db: Arc<dyn DatabaseProvider>,
}

impl std::fmt::Debug for LibsqlSessionStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibsqlSessionStore")
            .field("db", &"DatabaseProvider")
            .finish()
    }
}

impl LibsqlSessionStore {
    /// Create a new session store with database connection
    pub fn new(db: Arc<dyn DatabaseProvider>) -> Self {
        Self { db }
    }

    /// Create sessions table if it doesn't exist
    pub async fn migrate(&self) -> Result<()> {
        let conn = self.db.get_connection();

        // Create sessions table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                expiry_date INTEGER NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            )
            "#,
            params![],
        )
        .await?;

        // Create index for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_expiry ON sessions(expiry_date)",
            params![],
        )
        .await?;

        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(&self) -> Result<()> {
        let conn = self.db.get_connection();
        let now = OffsetDateTime::now_utc().unix_timestamp();

        conn.execute("DELETE FROM sessions WHERE expiry_date < ?", params![now])
            .await?;

        Ok(())
    }
}

#[async_trait]
impl SessionStore for LibsqlSessionStore {
    async fn create(&self, record: &mut Record) -> std::result::Result<(), Error> {
        let conn = self.db.get_connection();

        loop {
            let session_json =
                serde_json::to_string(&record.data).map_err(|e| Error::Backend(e.to_string()))?;

            let expiry_timestamp = record.expiry_date.unix_timestamp();

            // Try to insert the session
            match conn
                .execute(
                    "INSERT INTO sessions (id, data, expiry_date) VALUES (?, ?, ?)",
                    params![record.id.to_string(), session_json, expiry_timestamp],
                )
                .await
            {
                Ok(_) => return Ok(()),
                Err(_) => {
                    // ID collision or other error - generate new ID and retry
                    record.id = Id::default();
                    continue;
                }
            }
        }
    }

    async fn save(&self, record: &Record) -> std::result::Result<(), Error> {
        let conn = self.db.get_connection();

        let session_json =
            serde_json::to_string(&record.data).map_err(|e| Error::Backend(e.to_string()))?;

        let expiry_timestamp = record.expiry_date.unix_timestamp();

        conn.execute(
            "UPDATE sessions SET data = ?, expiry_date = ? WHERE id = ?",
            params![session_json, expiry_timestamp, record.id.to_string()],
        )
        .await
        .map_err(|e| Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> std::result::Result<Option<Record>, Error> {
        let conn = self.db.get_connection();

        let mut result = conn
            .query(
                "SELECT data, expiry_date FROM sessions WHERE id = ?",
                params![session_id.to_string()],
            )
            .await
            .map_err(|e| Error::Backend(e.to_string()))?;

        if let Ok(Some(row)) = result.next().await {
            let session_data: HashMap<String, serde_json::Value> =
                serde_json::from_str(row.get::<String>(0).unwrap_or_default().as_str())
                    .map_err(|e| Error::Backend(e.to_string()))?;

            let expiry_timestamp: i64 = row.get(1).unwrap_or(0);
            let expiry_date = OffsetDateTime::from_unix_timestamp(expiry_timestamp)
                .map_err(|_| Error::Backend("Invalid expiry timestamp".to_string()))?;

            // Check if session is expired
            if expiry_date < OffsetDateTime::now_utc() {
                // Delete expired session and return None
                let _ = conn
                    .execute(
                        "DELETE FROM sessions WHERE id = ?",
                        params![session_id.to_string()],
                    )
                    .await;
                return Ok(None);
            }

            Ok(Some(Record {
                id: *session_id,
                data: session_data,
                expiry_date,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> std::result::Result<(), Error> {
        let conn = self.db.get_connection();

        conn.execute(
            "DELETE FROM sessions WHERE id = ?",
            params![session_id.to_string()],
        )
        .await
        .map_err(|e| Error::Backend(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for LibsqlSessionStore {
    async fn delete_expired(&self) -> std::result::Result<(), Error> {
        self.cleanup_expired()
            .await
            .map_err(|e| Error::Backend(e.to_string()))
    }
}
