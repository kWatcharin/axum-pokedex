use serde::{ Serialize, Deserialize };


pub mod navbar {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Payload {
        pub username: String,
        pub password: String,
        pub is_login: bool,
        pub app_id: u8,
        pub app_type: u8,
        pub db: String
    }

    #[derive(Serialize, Debug)]
    pub struct Response {
        pub detail: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatePayload {
        pub name: String,
    }
} 