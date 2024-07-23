use sqlx::{Pool, postgres::Postgres};
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use crate::errors::{Error, Result};


pub mod poke_test {
    use super::*;
    use crate::db::pokemons::poke_test;
    use crate::models::pokemons::poke_test::api::{
        CreatePokemonPayload, UpdatePokeTestPayload
    };
    use crate::models::pokemons::poke_test::services::{
        PokeList, VecPokeList
    };
    use crate::models::pokemons::poke_test::db::{
        CreatePokeTest, UpdatePokeTest
    };

        
    pub async fn list(pool: &Pool<Postgres>) -> Result<(StatusCode, Json<VecPokeList>)> {
        match poke_test::list(pool).await {
            Ok(rows) => {
                let mut poke_data = Vec::new();

                for (_index, row) in rows.into_iter().enumerate() {
                    poke_data.push(PokeList {
                        rowid: row.rowid,
                        poke_code: row.poke_code,
                        poke_name: row.poke_name,
                        lv: row.lv,
                        create_date: row.create_date
                    });
                }
                
                Ok(
                    (StatusCode::OK, Json(VecPokeList::new(poke_data)))
                )
            },
            Err(_) => Err(Error::ServiceUnavailable)
        }        
    }


    pub async fn create(pool: &Pool<Postgres>, create_pokemon_payload: CreatePokemonPayload) -> Result<(StatusCode, Json<Value>)> {
        let create_new_poke_test = CreatePokeTest {
            poke_code: create_pokemon_payload.poke_code,
            poke_name: create_pokemon_payload.poke_name,
            lv: create_pokemon_payload.lv
        };

        let result = poke_test::create(pool, create_new_poke_test)
            .await;
        
        match result {
            Ok(rowcount) => {
                if rowcount > 0 {
                    Ok(
                        (StatusCode::CREATED, Json(json!({"detail": "create item successful"})))
                    )
                } else {
                    Err(Error::ServiceUnavailable)
                }
            },
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }


    pub async fn update(pool: &Pool<Postgres>, update_poke_test: UpdatePokeTestPayload) -> Result<(StatusCode, Json<Value>)> {
        let update_poke_test = UpdatePokeTest {
            poke_code: update_poke_test.poke_code,
            poke_name: update_poke_test.poke_name,
            lv: update_poke_test.lv,
            rowid: update_poke_test.rowid
        };

        let result = poke_test::update(pool, update_poke_test).await;

        match result {
            Ok(rowcount) => {
                if rowcount > 0 {
                    Ok(
                        (StatusCode::CREATED, Json(json!({"detail": "update item successful"})))
                    )
                } else {
                    Err(Error::ServiceUnavailable)
                }
            },
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }
}