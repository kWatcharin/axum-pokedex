use sqlx::{Pool, Postgres};


type Result<T, E = sqlx::error::Error> = core::result::Result<T, E>;

pub mod poke_test {
    use super::*;
    use crate::models::pokemons::poke_test::db::{
        CreatePokeTest, PokeTest, UpdatePokeTest
    };


    pub async fn list(pool: &Pool<Postgres>) -> Result<Vec<PokeTest>> {        
        let sql = format!(r#"
            SELECT 
                rowid, 
                poke_code, 
                poke_name, 
                lv,
                create_date
            FROM 
                poke_test
            ORDER BY 
                rowid;
        "#);
        
        let executor = sqlx::query_as::<Postgres, PokeTest>(&sql)
            .fetch_all(pool)
            .await?;

        Ok(executor)
    }

    
    pub async fn create(pool: &Pool<Postgres>, model: CreatePokeTest) -> Result<u64> {
        let sql = format!(r#"
            INSERT INTO poke_test(
                poke_code, poke_name, lv, create_date
            )
            VALUES(
                $1, $2, $3, now()
            );
        "#);

        let executor = sqlx::query(&sql)
            .bind(model.poke_code)
            .bind(model.poke_name)
            .bind(model.lv)
            .execute(pool)
            .await?;

        Ok(executor.rows_affected())
    }


    pub async fn update(pool: &Pool<Postgres>, model: UpdatePokeTest) -> Result<u64> {
        let sql = format!(r#"
            UPDATE poke_test SET
                poke_code = $1,
                poke_name = $2,
                lv = $3,
                create_date = now()
            WHERE rowid = $4
        "#);
        let executor = sqlx::query(&sql)
            .bind(model.poke_code)
            .bind(model.poke_name)
            .bind(model.lv)
            .bind(model.rowid)
            .execute(pool)
            .await?;

        Ok(executor.rows_affected())
    }
}