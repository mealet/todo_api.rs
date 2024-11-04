use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, sql::Thing};

#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub finished: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditTask {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckFinished {
    pub finished: bool
}
