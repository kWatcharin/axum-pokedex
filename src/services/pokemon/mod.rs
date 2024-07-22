use sqlx::{Pool, postgres::Postgres};
use axum::{http::StatusCode, Json};

pub mod poke_test {
    use serde_json::{json, Value};

    use super::*;
    use crate::db::pokemons::poke_test;
    use crate::errors::{Error, Result};
    use crate::models::pokemons::poke_test::api::CreatePokemonPayload;
    use crate::models::pokemons::poke_test::services::{
        PokeList, VecPokeList
    };
    use crate::models::pokemons::poke_test::db::CreateNewPokeTest;

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

    #[allow(unused)]
    pub async fn create_new(
        pool: &Pool<Postgres>, 
        create_pokemon_payload: CreatePokemonPayload
    ) -> Result<(StatusCode, Json<Value>)> {
        let create_new_poke_test = CreateNewPokeTest {
            poke_code: create_pokemon_payload.poke_code,
            poke_name: create_pokemon_payload.poke_name,
            lv: create_pokemon_payload.lv
        };

        let result = poke_test::create_new(pool, create_new_poke_test)
            .await;
        
        match result {
            Ok(v) => Ok((StatusCode::CREATED, Json(json!({"detail": "create item successful"})))),
            Err(err) => Err(Error::DatabaseQueryError)
        }
        
    }
}