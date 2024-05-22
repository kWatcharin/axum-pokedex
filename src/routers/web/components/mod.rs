mod apis;
use apis::navbar;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/navbar", navbar::router())
}