pub mod apis;
use apis::root;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/", root::router())
}


