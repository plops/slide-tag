use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: Option<i64>,
    pub name: String,
}
