use crate::feed::Feed;

use super::Store;
use rusqlite::params;

impl Store {
    pub fn add_feed(&mut self, feed_name: String, feed_url: String) {
        self.db
            .execute("INSERT INTO feeds (name, url) VALUES (?, ?)", params![feed_name, feed_url])
            .unwrap();
    }

    pub fn list_feeds(&mut self) -> Vec<Feed> {
        self.db
            .prepare("SELECT id, url, name FROM feeds")
            .unwrap()
            .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap(), row.get(2).unwrap())))
            .unwrap()
            .map(|row| {
                let (id, url, name) = row.unwrap();
                Feed { id, url, name }
            })
            .collect()
    }

}
