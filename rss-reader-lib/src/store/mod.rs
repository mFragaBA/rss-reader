use std::path::PathBuf;

use rusqlite::Connection;

mod migrations;
mod feed;

pub struct Store {
    pub(crate) db: Connection,
}

impl Store {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new instance of [Store] instantiated with the specified configuration options.
    pub fn new(config: StoreConfig) -> Self {
        let mut db = Connection::open(config.path).unwrap();
        migrations::update_to_latest(&mut db);

        Self { db }
    }
}

pub struct StoreConfig {
    pub path: String,
}

impl Default for StoreConfig {
    fn default() -> Self {
        const STORE_FILENAME: &str = "store.sqlite3";

        // get directory of the currently executing binary, or fallback to the current directory
        let exec_dir = match std::env::current_exe() {
            Ok(mut path) => {
                path.pop();
                path
            }
            Err(_) => PathBuf::new(),
        };

        let store_path = exec_dir.join(STORE_FILENAME);

        Self {
            path: store_path
                .into_os_string()
                .into_string()
                .expect("Creating the hardcoded path to the store file should not panic")
        }
    }
}
