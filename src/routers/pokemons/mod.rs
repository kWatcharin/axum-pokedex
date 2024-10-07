#![allow(unused)]
use std::sync::{Arc, RwLock};
use axum::{routing::post, response::IntoResponse, Router, Json, extract::State};
use tower_cookies::{Cookies, Cookie};
use sqlx::{Pool, postgres::Postgres};
use crate::models::main::db::ConnPools;
use crate::errors::{Result, Error};


pub fn router(pools: ConnPools) -> Router {
    Router::new()
        .nest("/poke_test", poke_test::router(pools.clone()))
}


mod poke_test {
    use super::*;
    use crate::services::pokemon::poke_test;
    use crate::models::pokemons::poke_test::api::{
        CreatePokemonPayload, UpdatePokeTestPayload
    };


    pub fn router(pools: ConnPools) -> Router {
        Router::new()
            .route("/list", post(list))
            .route("/create", post(create))
            .route("/update", post(update))
            .with_state(pools)
    }
    

    async fn list(State(pools): State<ConnPools>) -> Result<impl IntoResponse> {
        match pools.postgresql {
            Some(pg_pool) => {
                let pg_pool: Arc<RwLock<Pool<Postgres>>> = Arc::clone(&pg_pool);
                let pg_pool: Pool<Postgres> = pg_pool.read().unwrap().clone();
                Ok(
                    poke_test::list(&pg_pool)
                        .await?
                )
            },
            None => return Err(Error::InternalServerError)
        }
    }


    async fn create(State(pools): State<ConnPools>, payload: Json<CreatePokemonPayload>) -> Result<impl IntoResponse> {
        match pools.postgresql {
            Some(pg_pool) => {
                let pg_pool: Arc<RwLock<Pool<Postgres>>> = Arc::clone(&pg_pool);
                let pg_pool: Pool<Postgres> = pg_pool.read().unwrap().clone();
                let body = match payload {
                    Json(data) => data
                };
                Ok(
                    poke_test::create(&pg_pool, body)
                        .await?
                )
            },
            None => return Err(Error::InternalServerError)
        }
    }


    async fn update(State(pools): State<ConnPools>, payload: Json<UpdatePokeTestPayload>) -> Result<impl IntoResponse> {
        match pools.postgresql {
            Some(pg_pool) => {
                let pg_pool: Arc<RwLock<Pool<Postgres>>> = Arc::clone(&pg_pool);
                let pg_pool: Pool<Postgres> = pg_pool.read().unwrap().clone();
                let body = match payload {
                    Json(data) => data
                };
                Ok(
                    poke_test::update(&pg_pool, body)
                        .await?
                )
            },
            None => return Err(Error::InternalServerError)
        }
    }
}