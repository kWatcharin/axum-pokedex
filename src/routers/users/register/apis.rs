use axum::{ routing::{ get, post }, Router, Json };
use serde_json::{ Value, json };


/* Users Group Module */ 
pub mod users {
    use super::*;
    use crate::models::users::users::{
        GetRequest, GetResponse
    };
    use crate::services::users::users::test_loop_vecs;


    pub fn router() -> Router {
        Router::new()
            .route("/get", get(users::get_user))
            .route("/post", post(users::post_user))
    }

    async fn get_user() -> Json<Value> {
        test_loop_vecs();

        let res = GetResponse {
            detail: "detail".to_string()
        };
        Json(json!(res))
    }

    async fn post_user(Json(payload): Json<GetRequest>) -> Json<Value> {
        println!("{:?}", payload.name);

        Json(json!(GetResponse{
            detail: payload.name
        }))
    }
}


/* Register Module */ 
pub mod register {
    use super::*;


    pub fn router() -> Router {
        Router::new()
    }
}