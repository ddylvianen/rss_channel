use std::sync::Arc;
use crate::postgres::Postgres;
use axum::{routing::get, Router, Json};
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::rss_channel::ChannelArticles;
use crate::rss_channel::RssChannel;
use crate::repository::channel_repository::ChannelRepository;

mod postgres;
pub mod entity;
pub mod repository;
mod rss_channel;

#[tokio::main]
// 1 endpoint return nested JSON
async fn main() {
    let db = Arc::new(Postgres::new().await.unwrap());
    let app = Router::new()
        .route("/", get(get_news))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    channel_articles: Vec<ChannelArticles>
}

async fn get_news(State(db):  State<Arc<Postgres>>) -> Result<Json<Response>, StatusCode> {
    let repo = ChannelRepository::new(&db.connection);

    let channels = repo.get_all_channels().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let channel_articles = RssChannel::refresh(channels).await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(Response { channel_articles }))
}



