use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub url: String,
    pub channel_url: Option<String>
}

impl NewsArticle {
    pub fn new(id: Option<i32>, title: String, content: String, url: String, channel_url: Option<String>) -> Self {
        Self {
            id,
            title,
            content,
            url,
            channel_url,
        }
    }
}

impl From<&tokio_postgres::row::Row> for NewsArticle {
   fn from(row: &tokio_postgres::Row) -> Self {
       Self {
           id: row.get("id"),
           title: row.get("title"),
           content: row.get("content"),
           url: row.get("url"),
           channel_url: row.get("channel_url")
       }
   }

}