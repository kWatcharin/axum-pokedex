#![allow(unused)]
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde::Deserialize;
use serde::Serialize;
use url_encoded_data::stringify;
use serde_json::{json, Value};
use chrono::{prelude::*, Duration};
use dotenv::dotenv;
use std::env;
use std::fmt;
use std::fs;

use crate::configs::env::main::{
    SECRET_KEY, SERVICE_ACCOUNT, CLIENT_ID, CLIENT_SECRET
};


pub type Result<T, E = Box<dyn std::error::Error>> = core::result::Result<T, E>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: u8,
    pub sub: String,
    pub iat: i64,
    pub exp: i64
}

impl Claims {
    pub fn new(iss: u8, sub: String, iat: i64, exp: i64) -> Self {
        Self {
            iss, sub, iat, exp
        }
    }

    pub fn make_jwt() -> Result<String, Box<dyn std::error::Error>>{
        let mut header = Header::new(Algorithm::RS256); 
        header.typ = Some("JWT".to_string());

        let now = Local::now();
        let iat = now.timestamp();
        let exp = (now + Duration::hours(1)).timestamp();

        let claims = Self::new(
            CLIENT_ID.clone(), 
            SERVICE_ACCOUNT.clone(), 
            iat, 
            exp
        );

        let jwt = encode(&header, &claims, &EncodingKey::from_rsa_pem( SECRET_KEY.clone().as_bytes())?)?;
        Ok(jwt)
    }
}