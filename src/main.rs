mod configs; 
mod db;
mod errors;
mod middlewares;
mod models;
mod resources;
mod routers;
mod services;
mod utils;

use routers::fallback::not_found_api;
use routers::{
    auth, 
    root, 
    users, 
    web 
};

use axum::Router;
use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::Level;
use tower_http::trace::{ self, TraceLayer };


fn routers_index() -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/web", web::router())
        .fallback(not_found_api())
}
 
 
#[tokio::main]
async fn main() {
    let apis = routers_index()
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

    tracing_subscriber::fmt().with_target(false).compact().init();
    tracing::info!("🚀🌟 listening on port => {:?} 🚀🌟", configs::PORT);

    let listener = TcpListener::bind(
            SocketAddr::from(([0, 0, 0, 0], configs::PORT))
        ).await
        .unwrap();

    axum::serve(listener, apis.into_make_service())
        .await
        .unwrap();
}


