use axum::{ Router, routing::get, Json, http::StatusCode };
use serde_json::{ json, Value };


pub fn router() -> Router {
    Router::new()
        .route("/", get(greating()))
}

fn greating() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK, 
        Json(json!({ "detail": "Hello, welcome to axum-pokedex apis." }))
    )
}
