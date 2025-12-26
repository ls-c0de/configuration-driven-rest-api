// Create database File
// Create a pool on it

use sqlx::{Sqlite, SqlitePool, Pool};

pub async fn get_connection_to_db(db_url: &str) -> Pool<Sqlite> {
    println!("Connection established");
    SqlitePool::connect(db_url).await.unwrap()
}