use reqwest::Result;

pub async fn list_feeds() -> Result<()> {
    let request = reqwest::get("https://ntietz.com/atom.xml").await?;
    let text = request.text().await?;
    dbg!(text);

    Ok(())
}
