use serde::{ Serialize, Deserialize };
use validator::Validate;


pub mod users {
    use super::*;

    #[derive(Deserialize, Validate, Debug)]
    pub struct GetRequest {
        pub name: String
    }

    #[derive(Serialize, Debug)]
    pub struct GetResponse {
        pub detail: String
    }
}


pub mod register {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Request<T> {
        pub name: T
    }

    #[derive(Serialize, Debug)]
    pub struct Response<T> {
        pub detail: T
    }
}