use axum::{ routing::post, Router, Json, http::StatusCode };


pub mod authentication {
    use super::*;
    use crate::models::auth::login::{ LoginPayload, LoginResponse };
    use crate::services::auth::login::check as check_login;
    use crate::errors::auth::Result;
    
    pub fn router() -> Router {
        Router::new()
            .route("/login", post(login))
    }

    async fn login(Json(payload): Json<LoginPayload>) -> Result<(StatusCode, Json<LoginResponse>)> {
        check_login(payload)
    }
}