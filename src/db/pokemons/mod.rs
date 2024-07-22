use sqlx::{self, Pool, Postgres};


type Result<T, E = sqlx::error::Error> = core::result::Result<T, E>;
pub mod poke_test {
    use super::*;
    use crate::models::pokemons::poke_test::db::{
        PokeTest, CreateNewPokeTest
    };

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
        
        let executor = sqlx::query_as::<_, PokeTest>(&sql)
            .fetch_all(pool)
            .await?;

        Ok(executor)
    }

    pub async fn create_new(pool: &Pool<Postgres>, create_new_poke_test: CreateNewPokeTest) -> Result<u64> {
        let sql = format!(r#"
            INSERT INTO poke_test(
                poke_code, poke_name, lv, create_date
            )
            VALUES(
                $1, $2, $3, now()
            );
        "#);

        let executor = sqlx::query(&sql)
            .bind(create_new_poke_test.poke_code)
            .bind(create_new_poke_test.poke_name)
            .bind(create_new_poke_test.lv)
            .execute(pool)
            .await?;

        Ok(executor.rows_affected())
    }
}