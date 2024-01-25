use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

// MIGRATIONS
// ================================================================================================

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::new(vec![M::up(include_str!("store.sql")),]);
}

pub fn update_to_latest(conn: &mut Connection) {
    MIGRATIONS
        .to_latest(conn)
        .expect("should update to latest migration")
}
