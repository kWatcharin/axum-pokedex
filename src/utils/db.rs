#![allow(unused)]
use std::time::Duration;
use sqlx::{
    self, mysql::{MySqlPoolOptions, MySql}, postgres::{PgPoolOptions, Postgres}, Pool
};

type Result<T, E = sqlx::Error> = core::result::Result<T, E>;


pub async fn postgresql_pool(db_url: &str, db: &str, max_connection: u32, timeout: u64) -> Result<Pool<Postgres>> {
    Ok(
        match PgPoolOptions::new()
            .max_connections(max_connection)
            .acquire_timeout(Duration::from_secs(timeout))
            .connect(db_url)
            .await {
                Ok(pool) => {
                    tracing::info!("✅ Connect to the database ->> ({:#?}) is successful!", db);
                    pool
                },
                Err(err) => {
                    tracing::error!("🔥 Failed to connect the database ->> ({:#?}) :{:#?}.", db, err);
                    std::process::exit(1);
                }
            }
    )
}


pub async fn mysql_pool(db_url: &str, db: &str, max_connection: u32, timeout: u64) -> Result<Pool<MySql>> {
    Ok(
        match MySqlPoolOptions::new()
            .max_connections(max_connection)
            .acquire_timeout(Duration::from_secs(timeout))
            .connect(db_url)
            .await {
                Ok(pool) => {
                    tracing::info!("✅ Connect to the database ->> ({:#?}) is successful!", db);
                    pool
                },
                Err(err) => {
                    tracing::error!("🔥 Failed to connect the database ->> ({:#?}) :{:#?}.", db, err);
                    std::process::exit(1);
                }
            }
    )
}
