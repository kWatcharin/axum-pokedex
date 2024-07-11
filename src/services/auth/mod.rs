use axum::{http::StatusCode, Json};


pub mod login {
    use super::*;
    use crate::models::auth::login::{LoginPayload, DetailResponse, LoginResponse};
    use crate::errors::auth::{Result, Error};

    
    pub fn check(model: LoginPayload) -> Result<(StatusCode, Json<LoginResponse>)> {
        let response: Result<(StatusCode, Json<LoginResponse>)>;

        if model.username != "demo1" {
            response = Err(
                Error::InvalidUsername
            )

        } else if model.pwd != "welcome" {
            response = Err(
                Error::InvalidPassword
            )

        } else{
            let detail_response = DetailResponse { is_login: true };
            let login_response = LoginResponse { detail: detail_response };
            response = Ok(
                (
                    StatusCode::OK, Json(login_response)
                )
            )
        }

        response
    }
}