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
use sqlx::{Pool, postgres::Postgres};


pub fn index(pool: Pool<Postgres>) -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/auth", auth::router(pool))
        .nest("/users", users::router())
        .nest("/web", web::router())
        .fallback(fallback::not_found_api())
}