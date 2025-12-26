#[cfg(feature = "database")]
pub mod queries;
pub mod migrations;
pub mod pool;

use crate::db::pool::{get_connection_to_db};

const DB_URL: &str = "sqlite://data.db";

#[cfg(feature = "database")]
pub async fn db () {
    let conn = migrations::start_migrations(DB_URL).await;

    migrations::update_migrations(DB_URL, conn).await;
}