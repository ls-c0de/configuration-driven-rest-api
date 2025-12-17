// Create database File
// Create a pool on it

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};

const DB_URL: &str = "sqlite://data.db";

pub async fn create_database() {
    println!("Creating Database...");

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created db in {}", DB_URL),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

pub async fn get_connection_to_db() -> Pool<Sqlite> {
    println!("Connection established");
    SqlitePool::connect(DB_URL).await.unwrap()
}