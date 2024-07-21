use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use sqlx::FromRow;

pub mod poke_test {
    use super::*;

    pub mod api {
        #[allow(unused)]
        use super::*;
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
    }

    pub mod db {
        use super::*;

        #[derive(Debug, FromRow)]
        #[allow(unused)]
        pub struct PokeTest {
            pub rowid: i32,
            pub poke_code: String,
            pub poke_name: String,
            pub lv: i32,
            pub create_date: NaiveDate
        }
    }   
}