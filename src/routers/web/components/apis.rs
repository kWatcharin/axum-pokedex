use axum::{ 
    routing::post, Router, Json
};
use serde_json::{ 
    Value, json 
};


pub mod navbar {
    use super::*;
    use crate::models::web::components::navbar;
    use serde_json::to_string_pretty;

    pub fn router() -> Router {
        Router::new()
            .route("/", post(navbar))
    }

    async fn navbar(Json(payload): Json<navbar::Payload>) -> Json<Value> {
        println!("{:?}", payload);        
        let res = navbar::Response {
            detail: "Successful".to_string()
        };
        if to_string_pretty(&res).is_ok() {
            println!("{}", to_string_pretty(&res).ok().unwrap());
        }
        Json(json!(res))
    }
}