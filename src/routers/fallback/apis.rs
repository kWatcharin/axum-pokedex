use axum::{ http::StatusCode, Json };
use serde_json::{ json, Value };


pub fn not_found_api() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_FOUND, Json(json!({"detail": "not found"})))
}