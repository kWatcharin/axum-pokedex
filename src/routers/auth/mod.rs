use axum::{routing::post, http::StatusCode, response::IntoResponse, Router, Json, extract::State};
use tower_cookies::{Cookies, Cookie};
use sqlx::{Pool, postgres::Postgres};
use crate::errors::Result;


pub fn router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .nest("/login", login::router(pool))
}


mod login {
    use super::*;
    use crate::models::auth::login;
    use crate::db::pokemons::poke_test;

    
    pub fn router(pool: Pool<Postgres>) -> Router {
        Router::new()
            .route("/validate", post(validate))
            .with_state(pool)
    }

    async fn validate(
        State(pool): State<Pool<Postgres>>,
        cookies: Cookies, 
        _payload: Json<login::Payload>
    ) -> Result<impl IntoResponse> {
        cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));
        poke_test::fetch_all(&pool).await.unwrap();

        Ok(
            (
                StatusCode::OK,
                Json(
                    login::Response {
                        detail: String::from("successful!")
                    }
                )
            )
        )
    }
}