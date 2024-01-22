use reqwest::Result;
use rss::Channel;

pub async fn list_feeds() -> Result<()> {
    let request = reqwest::get("https://www.rssboard.org/files/sample-rss-2.xml").await?;
    let text = request.bytes().await?;

    let channel = Channel::read_from(&text[..]).unwrap();
    dbg!(channel);

    Ok(())
}
