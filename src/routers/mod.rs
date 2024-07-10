mod auth;
mod fallback;
mod items;
mod moves;
mod pokemons;
mod root;
mod setup;
mod users;
mod web;

use axum::Router;


pub fn index() -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/web", web::router())
        .fallback(fallback::not_found_api())
}