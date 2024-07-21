use sqlx::{Pool, postgres::Postgres};
use axum::{http::StatusCode, Json};

pub mod poke_test {
    use super::*;
    use crate::db::pokemons::poke_test;
    use crate::errors::{Error, Result};
    use crate::models::pokemons::poke_test::services::{
        PokeList, VecPokeList
    };

    pub async fn list(pool: &Pool<Postgres>) -> Result<(StatusCode, Json<VecPokeList>)> {
        match poke_test::fetch_all(pool).await {
            Ok(v) => {
                let mut poke_data = Vec::new();

                for poke in v.into_iter() {
                    poke_data.push(PokeList {
                        rowid: poke.rowid,
                        poke_code: poke.poke_code,
                        poke_name: poke.poke_name,
                        lv: poke.lv,
                        create_date: poke.create_date
                    })
                }
                
                Ok(
                    (
                        StatusCode::OK, 
                        Json(VecPokeList::new(poke_data))
                    )
                )
            },
            Err(_) => Err(Error::ServiceUnavailable)
        }        
    }
}