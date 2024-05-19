use axum::{ routing::{ get, post }, Router, Json };
use serde_json::{ Value, json };


/* Main Router: Users */ 
pub fn router() -> Router {
    Router::new()
        .nest("/tests", users::router())  
        .nest("/register", register::router())      
}


/* Users Group Module */ 
mod users {
    use super::*;
    use crate::routers::users::models::users::GetResponse;

    pub fn router() -> Router {
        Router::new()
            .route("/get", get(users::get_user))
            .route("/post", post(users::post_user))
    }

    async fn get_user() -> Json<Value> {
        let res = GetResponse {
            detail: "detail".to_string()
        };
        Json(json!(res))
    }
    
    async fn post_user() -> Json<Value> {
        Json(json!({ "detail": "post" }))
    }
}


/* Register Module */ 
mod register {
    use super::*;

    pub fn router() -> Router {
        Router::new()
    }
}