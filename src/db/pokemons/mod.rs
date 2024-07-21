use sqlx::{self, Pool, Postgres, FromRow};


pub mod poke_test {
    use super::*;

    #[derive(Debug, FromRow)]
    #[allow(unused)]
    pub struct PokeTest {
        pub rowid: i32,
        pub poke_code: String,
        pub poke_name: String,
        pub lv: i32
    }

    pub async fn fetch_all(pool: &Pool<Postgres>) -> core::result::Result<Vec<PokeTest>, sqlx::Error> {        
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

        Ok(poke_test)
    }
}