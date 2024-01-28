use crate::feed::Feed;

use super::Store;
use rusqlite::params;

impl Store {
    pub fn add_feed(&mut self, feed_url: &str) {
        self.db
            .execute("INSERT INTO feeds (url) VALUES (?)", params![feed_url])
            .unwrap();
    }

    pub fn remove_feed_by_id(&mut self, id: u64) {
        self.db
            .execute("DELETE FROM feeds WHERE id = (?)", params![id])
            .unwrap();
    }

    pub fn remove_feed_by_url(&mut self, url: &str) {
        self.db
            .execute("DELETE FROM feeds WHERE url = (?)", params![url])
            .unwrap();
    }

    pub fn list_feeds(&mut self) -> Vec<Feed> {
        self.db
            .prepare("SELECT id, url FROM feeds")
            .unwrap()
            .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
            .unwrap()
            .map(|row| {
                let (id, url) = row.unwrap();
                Feed { id, url }
            })
            .collect()
    }

}
