use sqlx::{self, Pool, Postgres};
use crate::errors::{Error, Result};


pub mod poke_test {
    use super::*;
    use crate::models::pokemons::poke_test::db::PokeTest;

    pub async fn fetch_all(pool: &Pool<Postgres>) -> Result<Vec<PokeTest>> {        
        let sql = format!(r#"
            SELECT 
                rowid, 
                poke_code, 
                poke_name, 
                lv,
                create_date
            FROM 
                poke_test
        "#);
        
        let poke_test = sqlx::query_as::<_, PokeTest>(&sql)
            .fetch_all(pool)
            .await;

        match poke_test {
            Ok(q) => Ok(q),
            Err(_) => Err(Error::DatabaseQueryError)
        }
    }
}