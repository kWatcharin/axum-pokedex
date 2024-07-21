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


    pub fn router(pools: ConnPools) -> Router {
        Router::new()
            .route("/list", post(list))
            .with_state(pools)
    }

    async fn list(State(pools): State<ConnPools>) -> Result<impl IntoResponse> {
        let postgresql_pool = match pools.postgresql {
            Some(p) => p,
            None => return Err(Error::InternalServerError)
        };

        Ok(
            poke_test::list(&postgresql_pool)
                .await?
        )
    }
}