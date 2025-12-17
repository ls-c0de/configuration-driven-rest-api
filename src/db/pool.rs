use sqlx::{Pool, SqlitePool};

pub type DbPool = Pool<SqlitePool>;

// pub async fn create_pool(db_url: &str) -> Result<DbPool, sqlx::Error> {

// }