use rss_reader_lib::list_feeds;

#[tokio::main]
async fn main() -> Result<(), String> {
    list_feeds().await.map_err(|err| err.to_string())
}
