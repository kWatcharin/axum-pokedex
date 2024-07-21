use sqlx::{
    mysql::{MySqlPoolOptions, MySql}, postgres::{PgPoolOptions, Postgres}, Pool
};


type Result<T, E = sqlx::error::Error> = core::result::Result<T, E>;

pub async fn postgres_pool(db_url: &str, db: &str, max_connection: u32) -> Result<Pool<Postgres>> {
    Ok(
        match PgPoolOptions::new()
            .max_connections(max_connection)
            .connect(db_url)
            .await {
                Ok(pool) => {
                    tracing::info!("✅ Connect to the database =>> ({:#?}) is successful!", db);
                    pool
                },
                Err(err) => {
                    tracing::error!("🔥 Failed to connect the database =>> ({:#?}) :{:#?}", db, err);
                    std::process::exit(1);
                }
            }
    )
} 

#[allow(unused)]
pub async fn mysql_pool(db_url: &str, db: &str, max_connection: u32) -> Result<Pool<MySql>> {
    Ok(
        match MySqlPoolOptions::new()
            .max_connections(max_connection)
            .connect(db_url)
            .await {
                Ok(pool) => {
                    tracing::info!("✅ Connect to the database =>> ({:#?}) is successful!", db);
                    pool
                },
                Err(err) => {
                    tracing::error!("🔥 Failed to connect the database =>> ({:#?}) :{:#?}", db, err);
                    std::process::exit(1);
                }
            }
    )
}


#[allow(unused)]
pub async fn mariadb_pool(db_url: &str, db: &str, max_connection: u32) -> Result<Pool<MySql>> {
    Ok(
        match MySqlPoolOptions::new()
            .max_connections(max_connection)
            .connect(db_url)
            .await {
                Ok(pool) => {
                    tracing::info!("✅ Connect to the database =>> ({:#?}) is successful!", db);
                    pool
                },
                Err(err) => {
                    tracing::error!("🔥 Failed to connect the database =>> ({:#?}) :{:#?}", db, err);
                    std::process::exit(1)
                }
        }
    )
}