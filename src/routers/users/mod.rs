use axum::{routing::{get, post}, Router, Json, http::StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use crate::errors::{Result, Error};


pub fn router() -> Router {
    Router::new()
        .nest("/login", login::router())
        .nest("/users", users::router())
}


mod login {
    use super::*;
    use crate::models::auth::login::LoginPayload;
    use crate::services::auth::login::check as check_login;


    pub fn router() -> Router {
        Router::new()
            .route("/check", post(check))
            .route("/test", post(test_res))
            .route("/handler", get(handler))
            .route("/test_err", get(test_err))
    }


    async fn check(Json(payload): Json<LoginPayload>) -> Result<impl IntoResponse> {
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
                    Json(json!({ "message": "successfull"}))
                )
            )
        }
    }

    async fn handler() -> Response {
        let response = (
            StatusCode::OK,
            Json(json!({ "detail": "test" }))
        );
        response.into_response()
    }

    async fn test_err() -> Result<()> {
        Err(
            Error::DatabaseQueryError
        )
    }
}


mod users {
    use super::*;
    use crate::models::users::users::{GetRequest, GetResponse};
    use crate::services::users::users::test_loop_vecs;


    pub fn router() -> Router {
        Router::new()
            .route("/get", get(get_user))
            .route("/post", post(post_user))
    }

    
    async fn get_user() -> Json<GetResponse> {
        Json(GetResponse {
            detail: "detail".to_string()
        })
    }

    async fn post_user(Json(paylaod): Json<GetRequest>) -> Json<Value> {
        test_loop_vecs();
        Json(json!(GetResponse {
            detail: paylaod.name
        }))
    }
}