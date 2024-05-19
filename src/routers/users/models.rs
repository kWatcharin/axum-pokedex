use serde::{ Serialize, Deserialize };


pub mod users {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetRequest {
        pub name: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetResponse {
        pub detail: String
    }
}


pub mod register {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub name: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub detail: String
    }
}