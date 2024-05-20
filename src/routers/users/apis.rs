use axum::{ routing::{ get, post }, Router, Json };
use serde_json::{ Value, json };


/* Users Group Module */ 
pub mod users {
    use super::*;
    use crate::routers::users::models::users::GetResponse;

    pub fn router() -> Router {
        Router::new()
            .route("/get", get(users::get_user))
            .route("/post", post(users::post_user))
    }

    async fn get_user() -> Json<Value> {
        let mut my_vec: Vec<u32> = Vec::new();
        for v in 0..=10 {
            my_vec.push(v);
        }
        println!("{:?}", my_vec);

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
pub mod register {
    use super::*;

    pub fn router() -> Router {
        Router::new()
    }
}