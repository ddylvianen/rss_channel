use tokio_postgres::{Client, Error};
use crate::entity::channel::Channel;

pub struct ChannelRepository<'a> {
    db: &'a Client,
}


impl<'a> ChannelRepository<'a> {
    pub fn new(db: &'a Client) -> Self {
        Self { db }
    }
    pub async fn get_all_channels(&self) -> Result<Vec<Channel>, Error> {
        let rows = self.db.query("SELECT id, url FROM Channel", &[]).await?;

        Ok(rows.iter().map(Channel::from).collect())
    }

    pub async fn save(&self, channel: &Channel) -> Result<u64, Error> {
        self.db.execute("INSERT INTO channel(url) VALUES ($1)", &[&channel.url]).await
    }

}