use serde::{Serialize, Deserialize};

pub mod login {
    use super::*;

    // Request
    #[derive(Deserialize, Debug)]
    pub struct LoginPayload {
        pub username: String,
        pub pwd: String
    }

    // Response
    #[derive(Serialize, Debug)]
    pub struct DetailResponse {
        pub is_login: bool
    }

    #[derive(Serialize, Debug)]
    pub struct LoginResponse {
        pub detail: DetailResponse
    }

    #[derive(Deserialize, Debug)]
    pub struct Payload {
        pub username: String,
        pub password: String
    }

    #[derive(Serialize, Debug)]
    pub struct Response {
        pub detail: String
    }
}