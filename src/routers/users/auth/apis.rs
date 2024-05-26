use axum::{
    routing::post, Router, Json, http::StatusCode
};


pub mod authentication {
    use super::*;
    use crate::models::auth::login::{ LoginPayload, DetailResponse, LoginResponse };
    use crate::errors::auth::{ Result, AuthError};
    
    pub fn router() -> Router {
        Router::new()
            .route("/login", post(login))
    }

    async fn login(Json(payload): Json<LoginPayload>) -> Result<(StatusCode, Json<LoginResponse>)> {
        println!("{:?}", payload);

        if payload.username != "demo1" || payload.pwd != "welcome" {
            return Err(AuthError::LoginFailed);
        }

        let detail_response = DetailResponse {
            is_login: true
        };
        let login_response = LoginResponse {
            detail: detail_response
        };
        Ok((StatusCode::OK, Json(login_response)))
    }
}