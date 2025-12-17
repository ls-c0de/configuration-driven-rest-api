#[cfg(feature = "database")]
pub mod queries;
pub mod migrations;
pub mod pool;

use crate::db::pool::{create_database, get_connection_to_db};

#[cfg(feature = "database")]
pub async fn db (){
    create_database().await;

    let connection = get_connection_to_db().await;

    let res = sqlx::query("
        CREATE TABLE IF NOT EXISTS users 
        ( id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(25) NOT NULL);
         ").execute(&connection).await.unwrap();

    println!("{:?}", res);
}