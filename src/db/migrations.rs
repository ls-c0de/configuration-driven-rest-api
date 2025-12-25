use crate::specification::spec::{Fields};
use sqlx::Sqlite;
use sqlx::Pool;

pub async fn start_migrations(db: Pool<Sqlite>) {
    // Read config
    let config = crate::config::loader::load_config();

    // FIXME: Check migrations have not been applied yet

    // Apply migrations
    for model in config.model {
        create_table(db.clone(), model.tablename, model.fields).await.unwrap(); // Clone needed?
    }
}

// Since sqlx does not support dynamic table creation, we build the query manually
// No Risk of SQL Injection here, since table names and fields are controlled by the provided yaml file
//
// FIXME: We probably need to split into CREATE TABLE and ALTER TABLE for adding fields later
//
pub async fn create_table(db: Pool<Sqlite>, table_name: String, fields: Vec<Fields<String>>) -> Result<(), sqlx::Error> {
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        panic!("Invalid table name: {}", table_name);
    }

    let columns = fields
        .into_iter()
        .map(|f| {
            //let nullable = if f.nullable { "" } else { " NOT NULL" }; // If we support nullable fields later
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