use crate::specification::spec::{Main, Fields};
use sqlx::Sqlite;
use sqlx::Pool;

pub async fn start_migrations(db: Pool<Sqlite>) {
    // Build migrations from config
    // in our case from defaults, other than that would be from loaded config
    let config = Main::test_values();

    // Check migrations
    // FIXME:

    // Apply migrations
    for model in config.model {
        create_table(db.clone(), model.tablename, model.fields).await.unwrap(); // Clone needed?
    }
}

// Since sqlx does not support dynamic table creation, we build the query manually
// No Risk of SQL Injection here, since table names and fields are controlled by the provided yaml file
pub async fn create_table(db: Pool<Sqlite>, table_name: String, fields: Vec<Fields<String>>) -> Result<(), sqlx::Error> {
    if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        panic!("Ungültiger Tabellenname");
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

    Ok(())
}