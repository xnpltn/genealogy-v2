use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub mod query;

pub async fn connect_to_db(
    db_path: &str,
) -> std::result::Result<SqlitePool, Box<dyn std::error::Error>> {
    if !Sqlite::database_exists(std::format!("{}/store.db", db_path).as_str())
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(std::format!("{}/store.db", db_path).as_str()).await?;
        let pool = SqlitePool::connect(std::format!("{}/store.db", db_path).as_str()).await?;
        Ok(pool)
    } else {
        let pool = SqlitePool::connect(std::format!("{}/store.db", db_path).as_str()).await?;
        Ok(pool)
    }
}
