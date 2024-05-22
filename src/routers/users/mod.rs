mod apis;
use apis::{
    users, register
};

use axum::Router;


/* Main Router: Users */ 
pub fn router() -> Router {
    Router::new()
        .nest("/tests", users::router())  
        .nest("/register", register::router())      
}