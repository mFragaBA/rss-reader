use reqwest::Result;
use rss::Channel;
use store::{Store, StoreConfig};

mod store;
mod feed;

pub async fn sample_feed() -> Result<()> {
    let request = reqwest::get("https://www.rssboard.org/files/sample-rss-2.xml").await?;
    let text = request.bytes().await?;

    let _channel = Channel::read_from(&text[..]).unwrap();

    Ok(())
}

pub async fn list_feeds() {
    let mut store = Store::new(StoreConfig::default());
    let feeds = store.list_feeds();
    println!("ID | URL");
    for feed in feeds {
        println!("{} | {}", feed.id, feed.url);
    }
}

pub async fn get_current_feeds() {
    let mut store = Store::new(StoreConfig::default());
    let feeds = dbg!(store.list_feeds());
    for feed in feeds {
        let request = reqwest::get(feed.url).await.unwrap();
        let text = request.bytes().await.unwrap();

        let channel = Channel::read_from(&text[..]).unwrap();

        for item in channel.items {
            let title = item.title().unwrap_or("-");
            let link = item.link().unwrap_or("-");
            let synopsis = item.description().unwrap_or("-");
            let author = item.author().unwrap_or("-");
            let pub_date = item.pub_date().unwrap_or("-");
            let html = item.content().map(|content| shorten(content, 40)).unwrap_or("-".to_string());

            print!("=============== {pub_date} ================\n{title}-{author}\n{synopsis}\n{link}\n{html}\n\n");
        }
    }
}

fn shorten(text: &str, max_chars: usize) -> String {
    if text.len() > max_chars {
        format!("{}...", text[..max_chars].to_string())
    } else {
        text.to_string()
    }
}

pub async fn add_feed(url: &str) {
    let mut store = Store::new(StoreConfig::default());
    store.add_feed(url);
}

pub async fn remove_feed_by_id(id: u64) {
    let mut store = Store::new(StoreConfig::default());
    store.remove_feed_by_id(id);
}

pub async fn remove_feed_by_url(url: &str) {
    let mut store = Store::new(StoreConfig::default());
    store.remove_feed_by_url(url);
}
