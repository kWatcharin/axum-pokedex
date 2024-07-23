use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use sqlx::FromRow;

pub mod poke_test {
    use super::*;

    pub mod api {
        #[allow(unused)]
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreatePokemonPayload {
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdatePokeTestPayload {
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32,
            pub rowid: i32
        }
    }

    pub mod services {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PokeList {
            pub rowid: i32,
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32,
            pub create_date: NaiveDate
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct VecPokeList {
            pub detail: Vec<PokeList>
        }

        impl VecPokeList {
            pub fn new(detail: Vec<PokeList>) -> Self {
                Self {
                    detail
                }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdatePokeTest {
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32,
            pub rowid: i32
        }
    }

    pub mod db {
        use super::*;

        #[derive(Debug, FromRow)]
        pub struct PokeTest {
            pub rowid: i32,
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32,
            pub create_date: NaiveDate
        }
        
        #[derive(Debug, FromRow)]
        pub struct CreatePokeTest {
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32
        }

        #[derive(Debug, FromRow)]
        pub struct UpdatePokeTest {
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32,
            pub rowid: i32
        }
    }   
}