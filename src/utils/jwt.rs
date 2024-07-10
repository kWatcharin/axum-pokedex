use serde::{Serialize, Deserialize};
#[allow(unused)]
use jsonwebtoken::{
    encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey
};
use chrono::{Duration, Utc};
#[allow(unused)]
use crate::configs::env::main::SECRET_KEY;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub username: String,
    pub password: String,
    pub datetime_now: i64,
    pub datetime_exp: i64,
}

impl Claims {
    #[allow(unused)]
    pub fn new(username: String, password: String) -> Self {
        let datetime_now = Utc::now();
        let datetime_exp = datetime_now + Duration::hours(1);

        Self {
            username,
            password,
            datetime_now: datetime_now.timestamp(),
            datetime_exp: datetime_exp.timestamp()
        }
    }
}