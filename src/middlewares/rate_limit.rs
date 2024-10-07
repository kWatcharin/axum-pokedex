use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::net::SocketAddr;
use axum::{
    http::Request,
    middleware::Next,
    response::IntoResponse,
    extract::ConnectInfo
};
use crate::errors::{Result, Error};

#[allow(unused)]
pub async fn get_user_ip_addr<B>(req: Request<B>) -> Result<String> {
    let remote_addr = req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0);

    if let Some(addr) = remote_addr {
        let ip = format!("{:#?}", addr.ip());
        Ok(ip)

    } else {
        Err(Error::InternalServerError)
    }
}

pub type Requests = Arc<Mutex<HashMap<String, (u32, Instant)>>>;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub requests: Requests,
    pub max_requests: u32,
    pub window: Duration
}

impl RateLimiter {
    #[allow(unused)]
    pub fn new(requests: Requests, max_requests: u32, window: Duration) -> Self {
        Self { requests, max_requests, window }
    }
}

#[allow(unused)]
pub async fn rate_limiter<B>(req: Request<B>, _next: Next, limiter: Arc<RateLimiter>) -> Result<impl IntoResponse> {
    let ip_addr = get_user_ip_addr(req).await?;

    let mut requests = limiter.requests 
        .lock()
        .await;

    let now = Instant::now();

    if let Some((count, timestamp)) = requests.get_mut(&ip_addr) {
        if now.duration_since(*timestamp) > limiter.window {
            *count = 1;
            *timestamp = now;

        } else if *count >= limiter.max_requests {
            return Err(Error::TooManyRequests);

        } else {
            *count += 1;
        }

    } else {
        requests.insert(ip_addr, (1, now));
    }

    drop(requests);

    Ok(())
}
