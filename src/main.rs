mod configs;
mod errors;
mod middlewares;
mod modules;
mod resources;
mod routers;
mod services;
mod utils;

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;


fn routers_index() -> Router {
    let users; 
        users = routers::users::apis::router();
    let not_found;
        not_found = routers::fallback::apis::not_found_api();

    Router::new()
        .nest("/users", users)
        .fallback(not_found)
}


#[tokio::main]
async fn main() {
    let apis = routers_index();
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 9000)))
        .await
        .unwrap();

    axum::serve(listener, apis.into_make_service())
        .await
        .unwrap()
}


