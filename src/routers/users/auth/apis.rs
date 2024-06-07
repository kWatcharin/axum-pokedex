use axum::{ routing::{get, post}, Router, Json, http::StatusCode };
use axum::response::{ IntoResponse, Response };
use serde_json::json;


pub mod authentication {
    use super::*;
    use crate::models::auth::login::{ LoginPayload, LoginResponse };
    use crate::services::auth::login::check as check_login;
    use crate::errors::auth::{ Result, Error };
    

    pub fn router() -> Router {
        Router::new()
            .route("/login", post(login))
            .route("/test_login", post(test_res))
            .route("/handler", get(handler))
    }

    async fn login(Json(payload): Json<LoginPayload>) -> Result<(StatusCode, Json<LoginResponse>)> {
        check_login(payload)
    }

    async fn test_res(Json(payload): Json<LoginPayload>) -> Result<impl IntoResponse> {
        if payload.username == "" && payload.pwd == "" {
            Err(
                Error::LoginFailed
            )

        } else {
            Ok(
                (
                    StatusCode::OK, 
                    Json(json!({ "message": "successful" }))
                )
            )
        }
    }

    async fn handler() -> Response {
        let response;
        response = (
            StatusCode::OK, Json(json!({ "detail": "test" }))
        );
            
        response.into_response()
    }
}