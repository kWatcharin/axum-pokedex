pub mod auth;
pub mod fallback;
pub mod items;
pub mod moves;
pub mod pokemons;
pub mod root;
pub mod setup;
pub mod users;
pub mod web;

use axum::Router;

pub fn index() -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/web", web::router())
        .fallback(fallback::not_found_api())
}