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
use crate::models::main::db::ConnPools;


pub fn index(pools: ConnPools) -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/auth", auth::router(pools.clone()))
        .nest("/pokemons", pokemons::router(pools.clone()))
        .nest("/users", users::router())
        .nest("/web", web::router())
        .fallback(fallback::not_found_api())
}