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
    UpdateItemFail,

    #[error("Too many requests.")]
    TooManyRequests
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
        let error_message;
        let error_response =  match self {
            Error::LoginFailed => {
                error_message = ErrorMessage { message: "Unauthorized.".to_string() };
                (
                    StatusCode::UNAUTHORIZED, Json(ErrorResponse { detail: error_message })
                )
            },
            Error::DatabaseQueryError => {
                error_message = ErrorMessage { message: "Database query error.".to_string() };
                (
                    StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { detail: error_message })
                )
            },
            Error::InvalidUsername => {
                error_message = ErrorMessage { message: "Invalid username.".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(ErrorResponse { detail: error_message })
                )
            },
            Error::InvalidPassword => {
                error_message = ErrorMessage { message: "Invalid password.".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(ErrorResponse { detail: error_message })
                )
            },
            Error::InvalidDataFormat => {
                error_message = ErrorMessage { message: "invalid data format.".to_string() };
                (
                    StatusCode::BAD_REQUEST, Json(ErrorResponse { detail: error_message })
                )
            }
            Error::NotFoundData => {
                error_message = ErrorMessage { message: "Not found data.".to_string() };
                (
                    StatusCode::NOT_FOUND, Json(ErrorResponse { 
                        detail: error_message 
                    })
                )
            },
            Error::InternalServerError => {
                error_message = ErrorMessage { message: "Internal server error.".to_string() };
                (
                    StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { detail: error_message })
                )
            },
            Error::ServiceUnavailable => {
                error_message = ErrorMessage { message: "Service unavailable.".to_string() };
                (
                    StatusCode::SERVICE_UNAVAILABLE, Json(ErrorResponse { detail: error_message })
                )
            },
            Error::CreateItemFail => {
                error_message = ErrorMessage { message: "Create item fail.".to_string() };
                (
                    StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorResponse {detail: error_message })
                )
            },
            Error::UpdateItemFail => {
                error_message = ErrorMessage { message: "Update item fail.".to_string() };
                (
                    StatusCode::UNPROCESSABLE_ENTITY, Json(ErrorResponse {detail: error_message })
                )
            },
            Error::TooManyRequests => {
                error_message = ErrorMessage { message: "To many requests".to_string() };
                (
                    StatusCode::TOO_MANY_REQUESTS, Json(ErrorResponse {detail: error_message})
                )
            }
        };

        error_response.into_response()
    }
}

