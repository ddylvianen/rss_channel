CREATE TABLE IF NOT EXISTS
    channel(
        id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        url varchar NOT NULL);


CREATE TABLE IF NOT EXISTS
    news_article(
        id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        title varchar NOT NULL, 
        content TEXT NOT NULL, 
        url varchar NOT NULL,
        channel_id INT NOT NULL,
        CONSTRAINT fk_news_article_channel
            FOREIGN KEY (channel_id) REFERENCES channel (id));



INSERT INTO channel(url)
values ('https://www.waterkant.net/rss'), ('https://dwtonline.com/rss');