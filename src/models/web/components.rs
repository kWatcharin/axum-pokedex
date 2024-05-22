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
    pub struct Dog {
        pub name: String,
        pub year_born: i32,
        pub owner: DogOwner
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DogOwner {
        pub first_name: String,
        pub last_name: String
    }
} 