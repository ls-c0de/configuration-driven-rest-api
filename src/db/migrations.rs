use crate::specification::spec::{Fields};
use sqlx::Sqlite;
use sqlx::Pool;
use sqlx::migrate::MigrateDatabase;
use crate::db::get_connection_to_db;

pub async fn start_migrations(db_url: &str) -> Pool<Sqlite> {
    create_database(db_url).await;
    
    // Read config
    let config = crate::config::loader::load_config();

    let connection = get_connection_to_db(db_url).await;

    // Apply migrations
    for model in config.model {
        create_table(connection.clone(), model.tablename, model.fields).await.unwrap(); // Clone needed?
    }

    connection
}

/// Drops the existing database and recreates it, applying all migrations from scratch
/// We need to drop the entire database since altering tables in SQLite is limited
/// We probably need to atleast backup the database in future versions
/// 
pub async fn update_migrations(db_url: &str, conn: Pool<Sqlite>) -> Pool<Sqlite> { // returning the connection is a bad design choice here, but we will keep it for now
    conn.close().await;
    
    println!("Deleting Database...");
    
    let path = db_url.strip_prefix("sqlite://").unwrap_or(db_url);

    if let Err(e) = std::fs::remove_file(path) {
        eprintln!("Failed to delete DB: {e}");
    }

    start_migrations(db_url).await
}

async fn create_database(db_url: &str) {
    println!("Creating Database...");

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Created db in {}", db_url),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

// Since sqlx does not support dynamic table creation, we build the query manually
// No Risk of SQL Injection here, since table names and fields are controlled by the provided yaml file
//
// We can't in SQLite, but can in Postgres:
// _We probably need to split into CREATE TABLE and ALTER TABLE for adding fields later._
//
async fn create_table(db: Pool<Sqlite>, table_name: String, fields: Vec<Fields<String>>) -> Result<(), sqlx::Error> {
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        panic!("Invalid table name: {}", table_name);
    }

    let columns = fields
        .into_iter()
        .map(|f| {
            format!("{} {}", f.name, f.datatype)
        })
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        table_name, columns
    );

    println!("Executing query: {}", query);

    sqlx::query(&query).execute(&db).await?;

    // FIXME:Error handling
    Ok(())
}