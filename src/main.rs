use rss_reader_lib::{remove_feed_by_id, remove_feed_by_url, add_feed, list_feeds, get_current_feeds};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    AddFeed {
        /// feed's url
        #[arg(short, long)]
        url: String,
    },
    RemoveFeed {
        /// feed's id
        #[arg(short, long, conflicts_with="url")]
        id: Option<u64>,
        /// feed's url
        #[arg(short, long, conflicts_with="id")]
        url: Option<String>,
    },
    ListFeeds,
    ShowFeeds
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::AddFeed { url } => add_feed(url).await,
        Commands::RemoveFeed { id: Some(id), url: _} => remove_feed_by_id(*id).await,
        Commands::RemoveFeed { id: _, url: Some(url)} => remove_feed_by_url(url).await,
        Commands::RemoveFeed { id: _, url: _} => println!("you must provide either id or url argument"),
        Commands::ListFeeds => list_feeds().await,
        Commands::ShowFeeds => get_current_feeds().await,
    }

    Ok(())
}
