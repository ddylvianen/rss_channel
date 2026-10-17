use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Option<i32>,
    pub url: String,
}

impl Channel {
 pub fn new(id: Option<i32>, url: String) -> Self {
    Self{ id, url }
 }
}

impl From<&tokio_postgres::Row> for Channel {
    fn from(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            url: row.get("url"),
        }
    }
}