mod components;
use components::router as components_router;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/components", components_router())
}