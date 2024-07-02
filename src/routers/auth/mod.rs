use axum::{routing::post, Router, Json, http::StatusCode};
use axum::response::IntoResponse;
use tower_cookies::Cookies;
use tower_cookies::Cookie;
use serde_json::json;
use crate::errors::auth::Result;


pub fn router() -> Router {
    Router::new()
        .nest("/login", login::router())
}


mod login {
    use super::*;
    use crate::models::auth::login::ValidatePayload;

    
    pub fn router() -> Router {
        Router::new()
            .route("/validate", post(validate))
    }

    async fn validate(cookies: Cookies, payload: Json<ValidatePayload>) -> Result<impl IntoResponse> {
        cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));
        println!("{:#}", payload.username);
        println!("{:#}", payload.password);

        Ok(
            (
                StatusCode::OK,
                Json(json!({ "detail": "successfull"}))
            )
        )
    }
}