use std::time::Duration;

use constants::DEFAULT_DB_URL;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbConn, TransactionTrait};

mod constants;
pub mod entities;
pub mod erase;
pub mod generate;
pub mod service;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or(DEFAULT_DB_URL.to_string());
    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(options).await.expect("Failed to setup the database");

    Migrator::up(&db, None).await.expect("Failed to run migrations");

    let tranasction = db.begin().await?;
    tranasction.commit().await?;

    Ok(db)
}
