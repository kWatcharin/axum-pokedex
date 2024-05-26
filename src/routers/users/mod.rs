mod auth;
mod register;

use auth::apis::authentication::router as auth_router;
use register::apis::{
    users::router as users_router,
    register::router as register_router
};

use axum::Router;


/* Main Router: Users */ 
pub fn router() -> Router {
    Router::new()
        .nest("/tests", users_router())  
        .nest("/register", register_router())
        .nest("/auth", auth_router())  
}