use sqlx::{Pool, postgres::Postgres};
use serde::{Serialize, Deserialize};
use axum::{http::StatusCode, Json};

pub mod poke_test {
    use super::*;
    use crate::db::pokemons::poke_test;
    use crate::errors::{Error, Result};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PokeList {
        pub rowid: i32,
        pub poke_code: String,
        pub poke_name: String,
        pub lv: i32
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VecPokeList {
        pub data: Vec<PokeList>
    }

    pub async fn list(pool: &Pool<Postgres>) -> Result<(StatusCode, Json<VecPokeList>)> {
        match poke_test::fetch_all(pool).await {
            Ok(v) => {
                let mut poke_data = Vec::new();

                for poke in v.into_iter() {
                    poke_data.push(PokeList {
                        rowid: poke.rowid,
                        poke_code: poke.poke_code,
                        poke_name: poke.poke_name,
                        lv: poke.lv
                    })
                }
                
                let result = VecPokeList {
                    data: poke_data
                };
                Ok((StatusCode::OK, Json(result)))
            },
            Err(_) => {
                Err(Error::DatabaseQueryError)
            }
        }        
    }
}