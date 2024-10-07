mod backend;
mod configs; 
mod db;
mod errors;
mod logs;
mod middlewares;
mod models;
mod resources;
mod routers;
mod schema;
mod services;
mod utils;

use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::Level;
use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    trace::{self, TraceLayer}
};
use tower_cookies::CookieManagerLayer;
use axum::http::{ header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE }, Method };
use sqlx::{postgres::Postgres, mysql::MySql, Pool};

use configs::env::{database::posgresql, main::PORT};
use models::main::db::ConnPools;
use utils::db::postgresql_pool; 

type Result<T, E = Box<dyn std::error::Error>> = core::result::Result<T, E>;


#[tokio::main]
async fn main() -> Result<()> {
    /* Start Tracing */
    tracing_subscriber::fmt().with_target(false).compact().init();

    /* Enveronments */
    configs::load_all_env();

    /* postgresql */
    let postgresql: Arc<RwLock<Pool<Postgres>>> = Arc::new(
        RwLock::new(
            postgresql_pool(
                &*posgresql::URL, &*posgresql::DB, *posgresql::MAX_CONNECTION, *posgresql::TIMEOUT
            ).await?
        )
    );

    /* mysql */
    let mysql: Option<Arc<RwLock<Pool<MySql>>>> 
        = None;

    /* mariadb */
    let mariadb: Option<Arc<RwLock<Pool<MySql>>>>
        = None;

    let pools = ConnPools::new(
        Some(postgresql), mysql, mariadb
    );

    let apis = routers::index(pools)
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        )
        .layer(
            CookieManagerLayer::new()
        )
        .layer(
            CompressionLayer::new()
                .gzip(true)
                .br(true)
                .deflate(true)
                .zstd(true)
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    trace::DefaultMakeSpan::new()
                        .level(Level::INFO)
                )
                .on_response(
                    trace::DefaultOnResponse::new()
                        .level(Level::INFO)
                )
                .on_failure(
                    trace::DefaultOnFailure::new()
                        .level(Level::ERROR)
                )
        );

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], *PORT))).await?;

    tracing::info!("✅ Start APIs Server, listening on port ->> {:#?} 🚀🌟.", *PORT);

    axum::serve(listener, apis.into_make_service()).await?;

    Ok(())
}


