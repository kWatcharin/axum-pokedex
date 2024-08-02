use sqlx::{Pool, postgres::Postgres};
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use crate::errors::{Result, Error};


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
            Ok(poke_test_rows) => {
                let mut poke_data = Vec::new();

                for (index, row) in poke_test_rows.into_iter().enumerate() {
                    poke_data.push(PokeList {
                        rowid: row.rowid,
                        poke_code: row.poke_code,
                        poke_name: row.poke_name,
                        lv: row.lv,
                        create_date: row.create_date,
                        idx: index
                    });
                }
                
                Ok(
                    (StatusCode::OK, Json(VecPokeList::new(poke_data)))
                )
            },
            Err(_) => Err(Error::DatabaseQueryError)
        }        
    }


    pub async fn create(pool: &Pool<Postgres>, schema: CreatePokemonPayload) -> Result<(StatusCode, Json<Value>)> {
        let model = CreatePokeTest::new(
            schema.poke_code, schema.poke_name, schema.lv
        );
        
        match poke_test::create(pool, model).await {
            Ok(count_affected) => {
                if count_affected > 0 {
                    Ok(
                        (StatusCode::CREATED, Json(json!({"detail": "create item successful"})))
                    )
                } else {
                    Err(Error::CreateItemFail)
                }
            },
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }


    pub async fn update(pool: &Pool<Postgres>, schema: UpdatePokeTestPayload) -> Result<(StatusCode, Json<Value>)> {
        let model = UpdatePokeTest::new(
            schema.poke_code, schema.poke_name, schema.lv, schema.rowid
        );

        match poke_test::update(pool, model).await {
            Ok(count_affected) => {
                if count_affected > 0 {
                    Ok(
                        (StatusCode::CREATED, Json(json!({"detail": "update item successful"})))
                    )
                } else {
                    Err(Error::UpdateItemFail)
                }
            },
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }
}