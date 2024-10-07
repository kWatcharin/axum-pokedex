use std::sync::{Arc, RwLock};
use axum::{routing::post, response::IntoResponse, Router, Json, extract::State};
#[allow(unused)]
use tower_cookies::{Cookies, Cookie};
use sqlx::{Pool, postgres::Postgres};
use crate::errors::Result;
use crate::models::main::db::ConnPools;


pub fn router(pool: ConnPools) -> Router {
    Router::new()
        .nest("/login", login::router(pool.clone()))
}


mod login {
    use super::*;
    use crate::models::auth::login;
    use crate::services::pokemon::poke_test;

    
    pub fn router(pool: ConnPools) -> Router {
        Router::new()
            .route("/validate", post(validate))
            .with_state(pool)
    }

    async fn validate(
        State(pool): State<ConnPools>, cookies: Cookies, _payload: Json<login::Payload>
    ) -> Result<impl IntoResponse> {
        cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));
        let pg_pool: Arc<RwLock<sqlx::Pool<Postgres>>> = Arc::clone(&pool.postgresql.unwrap());
        let pg_pool: Pool<Postgres> = pg_pool.read().unwrap().clone();
        Ok(poke_test::list(&pg_pool).await?)
    }
}