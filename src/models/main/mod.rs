use sqlx::{Pool, postgres::Postgres, mysql::MySql};


pub mod db {
    use super::*;
    
    #[derive(Debug, Clone)]
    pub struct ConnPools {
        pub postgresql: Option<Pool<Postgres>>,
        pub mysql: Option<Pool<MySql>>,
        pub mariadb: Option<Pool<MySql>>
    }

    impl ConnPools {
        pub fn new(postgresql: Option<Pool<Postgres>>, mysql: Option<Pool<MySql>>, mariadb: Option<Pool<MySql>>) -> Self {
            Self {
                postgresql,
                mysql,
                mariadb
            }
        }
    }
}