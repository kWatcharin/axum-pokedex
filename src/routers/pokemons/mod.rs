#![allow(unused)]
use axum::{routing::post, response::IntoResponse, Router, Json, extract::State};
use tower_cookies::{Cookies, Cookie};
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
        let postgresql_pool = match &pools.postgresql {
            Some(pool) => pool,
            None => return Err(Error::InternalServerError)
        };

        Ok(
            poke_test::list(postgresql_pool)
                .await?
        )
    }


    async fn create(pools: State<ConnPools>, payload: Json<CreatePokemonPayload>) -> Result<impl IntoResponse> {
        let postgresql_pool = match &pools.postgresql {
            Some(pool) => pool,
            None => return Err(Error::InternalServerError)
        };

        let body = CreatePokemonPayload {
            poke_code: payload.poke_code.to_owned(),
            poke_name: payload.poke_name.to_owned(),
            lv: payload.lv.to_owned()
        };

        Ok(
            poke_test::create(postgresql_pool, body)
                .await?
        )
    }


    async fn update(pools: State<ConnPools>, payload: Json<UpdatePokeTestPayload>) -> Result<impl IntoResponse> {
        let postgresql_pool = match &pools.postgresql {
            Some(pool) => pool,
            None => return Err(Error::InternalServerError)
        };

        let body = UpdatePokeTestPayload {
            poke_code: payload.poke_code.to_owned(),
            poke_name: payload.poke_name.to_owned(),
            lv: payload.lv.to_owned(),
            rowid: payload.rowid.to_owned()
        };

        Ok(
            poke_test::update(postgresql_pool, body)
                .await?
        )
    }
}