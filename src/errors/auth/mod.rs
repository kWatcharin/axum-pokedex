use thiserror::Error as ThisError;
use axum::{ http::StatusCode, response::{ IntoResponse, Response}, Json };
use serde::{Deserialize, Serialize};


#[derive(ThisError, Debug)]
pub enum Error {
    #[allow(unused)]
    #[error("Failed to login")]
    LoginFailed,

    #[allow(unused)]
    #[error("Database query error")]
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

pub type Result<T, E = Error> = axum::response::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
struct AuthErrorMessage {
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthErrorResponse {
    detail: AuthErrorMessage
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let auth_error_message;

        let error_response =  match self {
            Error::LoginFailed => {
                auth_error_message = AuthErrorMessage { message: "unauthorized".to_string() };
                (
                    StatusCode::UNAUTHORIZED, Json(AuthErrorResponse { detail: auth_error_message })
                )
            },

            Error::DatabaseQueryError => {
                auth_error_message = AuthErrorMessage { message: "database query error".to_string() };
                (
                    StatusCode::INTERNAL_SERVER_ERROR, Json(AuthErrorResponse { detail: auth_error_message })
                )
            },

            Error::InvalidUsername => {
                auth_error_message = AuthErrorMessage { message: "invalid username".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(AuthErrorResponse { detail: auth_error_message })
                )
            },

            Error::InvalidPassword => {
                auth_error_message = AuthErrorMessage { message: "invalid password".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(AuthErrorResponse { detail: auth_error_message })
                )
            },

            Error::InvalidDataFormat => {
                auth_error_message = AuthErrorMessage { message: "invalid data format".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(AuthErrorResponse { detail: auth_error_message })
                )
            }

            Error::NotFoundData => {
                auth_error_message = AuthErrorMessage { message: "not found data".to_string() };
                (
                    StatusCode::NOT_FOUND, Json(AuthErrorResponse { detail: auth_error_message })
                )
            }
        };

        error_response.into_response()
    }
}

