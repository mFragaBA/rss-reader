use rss_reader_lib::{sample_feed, add_feed, list_feeds, get_current_feeds};

#[tokio::main]
async fn main() -> Result<(), String> {
    // add_feed("test feed".to_string(), "https://www.rssboard.org/files/sample-rss-2.xml".to_string()).await;
    // add_feed("test feed 2".to_string(), "https://www.rssboard.org/files/sample-rss-2.xml".to_string()).await;
    // add_feed("test feed 3".to_string(), "https://www.rssboard.org/files/sample-rss-2.xml".to_string()).await;
    Ok(get_current_feeds().await)
}
