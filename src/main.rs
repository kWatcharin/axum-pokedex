mod configs; 
mod db;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod modules;
mod resources;
mod routers;
mod services;
mod utils;

use routers::fallback::apis::not_found_api as not_found;
use routers::{
    root, users, web 
};

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::Level;
use tower_http::trace::{ self, TraceLayer };


fn routers_index() -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/users", users::router())
        .nest("/web", web::router())
        .fallback(not_found())
}
 
 
#[tokio::main]
async fn main() {
    let apis = routers_index()
        .layer(
            middlewares::cors::cors_layor()
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

    tracing_subscriber::fmt().with_target(false).compact().init();
    tracing::info!("🚀🌟 listening on port => {:?} 🚀🌟", configs::settings::PORT);

    let listener = TcpListener::bind(
            SocketAddr::from(([0, 0, 0, 0], configs::settings::PORT))
        ).await
        .unwrap();

    axum::serve(listener, apis.into_make_service())
        .await
        .unwrap();
}


