use axum::{ 
    routing::{post, put}, Router, Json, http::StatusCode
};


pub mod navbar {
    use super::*;
    use serde_json::{json, Value};
    
    use crate::models::web::components::navbar::{
        Payload, Response, 
        CreatePayload
    };
    use crate::services::web::components::navbar::navbar_string;

    
    pub fn router() -> Router {
        Router::new()
            .route("/", post(navbar))
            .route("/create", post(create))
            .route("/update", put(update))
    }

    async fn navbar(Json(_payload): Json<Payload>) -> (StatusCode, Json<Response>) {
        let message: String = navbar_string();
        (
            StatusCode::OK, 
            Json(navbar::Response { 
                detail: message
            })
        )
    }

    async fn create(Json(payload): Json<CreatePayload>) -> (StatusCode, Json<CreatePayload>) {
        (StatusCode::CREATED, Json(payload))
    }

    async fn update() -> (StatusCode, Json<Value>) {
        (StatusCode::OK, Json(json!({"detail": "test"})))
    }
}