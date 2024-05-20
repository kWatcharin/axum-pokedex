use tower_http::cors::CorsLayer;
use axum::http::{ 
    header::{ 
        ACCEPT, AUTHORIZATION, CONTENT_TYPE 
    }, Method 
};


pub fn cors_layor() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
}