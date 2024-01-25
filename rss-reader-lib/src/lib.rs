use feed::Feed;
use reqwest::Result;
use rss::Channel;
use store::{Store, StoreConfig};

mod store;
mod feed;

pub async fn sample_feed() -> Result<()> {
    let request = reqwest::get("https://www.rssboard.org/files/sample-rss-2.xml").await?;
    let text = request.bytes().await?;

    let channel = Channel::read_from(&text[..]).unwrap();
    dbg!(channel);

    Ok(())
}

pub async fn list_feeds() {
    let mut store = Store::new(StoreConfig::default());
    let feeds = dbg!(store.list_feeds());
    for feed in feeds {
        println!("{} | {} | {}", feed.id, feed.name, feed.url)
    }
}

pub async fn get_current_feeds() {
    let mut store = Store::new(StoreConfig::default());
    let feeds = dbg!(store.list_feeds());
    for feed in feeds {
        let request = reqwest::get(feed.url).await.unwrap();
        let text = request.bytes().await.unwrap();

        let channel = Channel::read_from(&text[..]).unwrap();
        dbg!(channel);
    }
}

pub async fn add_feed(name: String, url: String) {
    let mut store = Store::new(StoreConfig::default());
    store.add_feed(name, url);
}
