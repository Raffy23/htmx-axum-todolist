use sqlx::{migrate::MigrateDatabase as _, Pool, Sqlite, SqlitePool};
use tracing::info;


pub async fn open_database(uri: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(uri).await.unwrap_or(false) {
        info!("Creating database {} ...", uri);

        match Sqlite::create_database(uri).await {
            Ok(_) => info!("Database created successfully!"),
            Err(error) => panic!("error: {}", error),
        }
    }

    SqlitePool::connect(uri).await
}
