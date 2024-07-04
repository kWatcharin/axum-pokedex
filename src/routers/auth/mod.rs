use axum::{
    routing::post, http::StatusCode, response::IntoResponse, Router, Json
};
use tower_cookies::{
    Cookies, Cookie
};
use crate::errors::auth::Result;


pub fn router() -> Router {
    Router::new()
        .nest("/login", login::router())
}


mod login {
    use super::*;
    use crate::models::auth::login::{
        ValidatePayload, ValidateResponse
    };

    
    pub fn router() -> Router {
        Router::new()
            .route("/validate", post(validate))
    }

    async fn validate(cookies: Cookies, _payload: Json<ValidatePayload>) -> Result<impl IntoResponse> {
        cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));

        Ok(
            (
                StatusCode::OK,
                Json(
                    ValidateResponse {
                        detail: String::from("successful!")
                    }
                )
            )
        )
    }
}