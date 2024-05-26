use thiserror::Error as ThisError;
use axum::{ http::StatusCode, response::{ IntoResponse, Response}, Json };
use serde_json::json;


#[derive(ThisError, Debug)]
pub enum AuthError {
    #[error("Failed ot login")]
    LoginFailed,

    #[allow(unused)]
    #[error("Failed to Query")]
    DatabaseQueryError,

    #[allow(unused)]
    #[error("Invalid username")]
    InvalidUsername,

    #[allow(unused)]
    #[error("Invalid password")]
    InvalidPassword,

    #[allow(unused)]
    #[error("Invalid data format")]
    InvalidDataFormat,

    #[allow(unused)]
    #[error("Not found data")]
    NotFoundData
}

pub type Result<T> = core::result::Result<T, AuthError>;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_response =  match self {
            AuthError::LoginFailed => {
                (StatusCode::UNAUTHORIZED, Json(json!({"detail": {"is_login": false}})))
            },
            AuthError::DatabaseQueryError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"detail": {"is_login": false}})))
            },
            _ => {
                (StatusCode::BAD_REQUEST, Json(json!({"detail": {"is_login": false}})))
            }
        };

        error_response.into_response()
    }
}

