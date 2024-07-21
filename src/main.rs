mod configs; 
mod db;
mod errors;
mod middlewares;
mod models;
mod resources;
mod routers;
mod services;
mod utils;

use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::Level;
use tower_http::trace::{self, TraceLayer};
use configs::env::{database::posgresql, main::PORT};
use utils::db::postgres_pool; 
use models::main::db::ConnPools;
use sqlx::{Pool, mysql::MySql};


#[tokio::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /* Start Tracing */
    tracing_subscriber::fmt().with_target(false).compact().init();

    /* Enveronments */
    configs::load_all_env();

    /* postgresql */
    let postgresql 
        = postgres_pool(&*posgresql::URL, &*posgresql::DB, *posgresql::MAX_CONNECTION)
        .await?;

    /* mysql */
    let mysql: Option<Pool<MySql>>
        = None;

    /* mariadb */
    let mariadb: Option<Pool<MySql>>
        = None;

    let pools = ConnPools::new(
        Some(postgresql), mysql, mariadb
    );

    let apis = routers::index(pools)
        .layer(
            middlewares::cors_layor()
        )
        .layer(
            CookieManagerLayer::new()
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

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], *PORT)))
        .await
        .unwrap();

    tracing::info!("✅ Start APIs Server, listening on port =>> {:#?} 🚀🌟", *PORT);

    axum::serve(listener, apis.into_make_service())
        .await
        .unwrap();

    Ok(())
}


