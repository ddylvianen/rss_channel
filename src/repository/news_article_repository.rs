use tokio_postgres::{Client, Error};
use crate::entity::channel::Channel;
use crate::entity::news_article::NewsArticle;

pub struct NewsArticleRepository<'a> {
    db: &'a Client,
}

impl<'a> NewsArticleRepository<'a> {
    fn new(db: &'a Client) -> Self {
        Self { db }
    }

    pub async fn save(&self, news_article: &NewsArticle, channel_url: String) -> Result<(), Error> {
        let row = self.db.query_one("SELECT id, url FROM channel WHERE url=$1", &[&channel_url]).await?;
        let channel = Channel::from(&row);

        self.db.query_opt("INSERT INTO news_article (title, content, url, channel_id) VALUES ($1, $2, $3, $4)",
                     &[&news_article.title, &news_article.content, &news_article.url, &channel.id],).await?;

        Ok(())
    }

    pub async fn get_news_articles(&self, db: Client) -> Result<Vec<NewsArticle>, Error> {
        let res = db.query("SELECT news.id, news.title, news.content, news.url, channel.url AS channel_url \
                                                FROM news_article as news \
                                                JOIN channel ON channel.id = news.channel_id", &[])
                            .await?;

        Ok(res.iter().map(NewsArticle::from).collect())
    }

    // pub fn purge(&self) {
    //     self.db.execute("PRUNE news_article", &[]).await;
    // }

}

