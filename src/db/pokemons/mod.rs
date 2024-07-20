use sqlx::{self, Pool, Postgres, FromRow};


pub mod poke_test {
    use super::*;

    #[derive(Debug, FromRow)]
    #[allow(unused)]
    struct PokeTest {
        rowid: i32,
        poke_code: String,
        poke_name: String,
        lv: i32
    }

    pub async fn fetch_all(pool: &Pool<Postgres>) -> core::result::Result<(), sqlx::Error> {        
        let sql = format!(
            r#"
                SELECT 
                    rowid, 
                    poke_code, 
                    poke_name, 
                    lv 
                FROM poke_test
            "#
        );
        
        let poke_test: Vec<PokeTest> = sqlx::query_as::<_, PokeTest>(&sql)
            .fetch_all(pool)
            .await?;

        for r in poke_test.iter() {
            println!("{:?}", r);
        }

        Ok(())
    }
}