-- Create feeds table
CREATE TABLE feeds (
    id INTEGER PRIMARY KEY,     -- id of the rss feed.
    url BLOB NOT NULL           -- url used to fetch the rss feed
);
