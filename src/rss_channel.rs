use rss::Channel as rss_channel;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use crate::entity::channel::{Channel as channel_entity, Channel};
use crate::entity::news_article::NewsArticle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelArticles {
    articles: Vec<NewsArticle>,
    channel: channel_entity,
}

impl ChannelArticles {
    fn new(articles: Vec<NewsArticle>, channel: channel_entity) -> ChannelArticles {
        Self { articles, channel }
    }
}

pub struct RssChannel {
}

impl RssChannel {

    pub async fn refresh(channel_entity: Vec<Channel>) -> Result<Vec<ChannelArticles>, Box<dyn std::error::Error>> {
        let mut channel_articles = Vec::new();

        for channel in channel_entity {
            let feed = reqwest::get(&channel.url).await?.text().await?;
            let channel_feed = rss_channel::read_from(feed.as_bytes())?;

            let mut news_articles = Vec::new();
            for item in channel_feed.items {
                let content = item.content.or(item.description).unwrap_or_default();

                let news = NewsArticle::new(Option::None, item.title.unwrap_or_default(), strip_html(&content), item.link.unwrap_or_default(), Option::from(channel.url.clone()));
                news_articles.push(news);
            }

            channel_articles.push(ChannelArticles::new(news_articles, channel.clone()));
        }

        Ok(channel_articles)
    }

}

fn strip_html(html: &str) -> String {
    let document = Html::parse_fragment(html);
    let paragraph_selector = Selector::parse("p").unwrap();

    let paragraphs: Vec<String> = document
        .select(&paragraph_selector)
        // pullquotes and image captions repeat or describe the article, they aren't part of it
        .filter(|p| !p.ancestors().filter_map(ElementRef::wrap).any(|a| matches!(a.value().name(), "figure" | "blockquote")))
        // a paragraph that is only a link is the site name WordPress puts above the article
        .filter(|p| !is_only_link(p))
        .map(|p| normalize_whitespace(&p.text().collect::<String>()))
        // WordPress footer: "Het bericht <titel> verscheen eerst op <site>."
        .filter(|text| !(text.starts_with("Het bericht ") && text.contains(" verscheen eerst op ")))
        .filter(|text| !text.is_empty())
        .collect();

    // feeds with a plain-text description have no <p> elements
    if paragraphs.is_empty() {
        return normalize_whitespace(&document.root_element().text().collect::<String>());
    }

    paragraphs.join("\n\n")
}

fn is_only_link(paragraph: &ElementRef) -> bool {
    let link_selector = Selector::parse("a").unwrap();
    let link_text: String = paragraph.select(&link_selector).flat_map(|a| a.text()).collect();
    let paragraph_text: String = paragraph.text().collect();

    !link_text.trim().is_empty() && link_text.trim() == paragraph_text.trim()
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
