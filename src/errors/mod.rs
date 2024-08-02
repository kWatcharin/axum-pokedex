#![allow(unused)]
use serde_json::error;
use thiserror::Error as ThisError;
use axum::{ http::StatusCode, response::{ IntoResponse, Response}, Json };
use serde::{Deserialize, Serialize};


pub type Result<T, E = Error> = axum::response::Result<T, E>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to login.")]
    LoginFailed,

    #[error("Database query error.")]
    DatabaseQueryError,

    #[error("Invalid username.")]
    InvalidUsername,

    #[error("Invalid password.")]
    InvalidPassword,

    #[error("Invalid data format.")]
    InvalidDataFormat,

    #[error("Not found data.")]
    NotFoundData,

    #[error("Internal server error.")]
    InternalServerError,

    #[error("Service unavailable.")]
    ServiceUnavailable,

    #[error("Create item fail.")]
    CreateItemFail,

    #[error("Update item fail")]
    UpdateItemFail
}


#[derive(Serialize, Deserialize, Debug)]
struct ErrorMessage {
    message: String
}

impl ErrorMessage {
    fn new(message: String) -> Self {
        Self { message }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    detail: ErrorMessage
}

impl ErrorResponse {
    fn new(detail: ErrorMessage) -> Self {
        Self { detail }
    }
}


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let auth_error_message;
        let error_response =  match self {
            Error::LoginFailed => {
                auth_error_message = ErrorMessage { message: "unauthorized.".to_string() };
                (
                    StatusCode::UNAUTHORIZED, Json(ErrorResponse { detail: auth_error_message })
                )
            },
            Error::DatabaseQueryError => {
                auth_error_message = ErrorMessage { message: "database query error.".to_string() };
                (
                    StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { detail: auth_error_message })
                )
            },
            Error::InvalidUsername => {
                auth_error_message = ErrorMessage { message: "invalid username.".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(ErrorResponse { detail: auth_error_message })
                )
            },
            Error::InvalidPassword => {
                auth_error_message = ErrorMessage { message: "invalid password.".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(ErrorResponse { detail: auth_error_message })
                )
            },
            Error::InvalidDataFormat => {
                auth_error_message = ErrorMessage { message: "invalid data format.".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(ErrorResponse { detail: auth_error_message })
                )
            }
            Error::NotFoundData => {
                auth_error_message = ErrorMessage { message: "not found data.".to_string() };
                (
                    StatusCode::NOT_FOUND, Json(ErrorResponse { 
                        detail: auth_error_message 
                    })
                )
            },
            Error::InternalServerError => {
                auth_error_message = ErrorMessage { message: "internal server error.".to_string() };
                (
                    StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { detail: auth_error_message })
                )
            },
            Error::ServiceUnavailable => {
                auth_error_message = ErrorMessage { message: "service unavailable.".to_string() };
                (
                    StatusCode::SERVICE_UNAVAILABLE, Json(ErrorResponse { detail: auth_error_message })
                )
            },
            Error::CreateItemFail => {
                auth_error_message = ErrorMessage { message: "create item fail.".to_string() };
                (
                    StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorResponse {detail: auth_error_message })
                )
            },
            Error::UpdateItemFail => {
                auth_error_message = ErrorMessage { message: "update item fail.".to_string() };
                (
                    StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorResponse {detail: auth_error_message })
                )
            }
        };

        error_response.into_response()
    }
}

