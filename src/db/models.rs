use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub finished: bool
}
