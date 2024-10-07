use std::sync::{Arc, RwLock};
use sqlx::{Pool, postgres::Postgres, mysql::MySql};


pub mod db {
    use super::*;

    pub type PgOption = Option<Arc<RwLock<Pool<Postgres>>>>;
    pub type MsOption = Option<Arc<RwLock<Pool<MySql>>>>;
    
    #[derive(Debug, Clone)]
    pub struct ConnPools {
        pub postgresql: PgOption,
        pub mysql: MsOption,
        pub mariadb: MsOption
    }

    impl ConnPools {
        pub fn new(
            postgresql: PgOption, 
            mysql: MsOption, 
            mariadb: MsOption
        ) -> Self {
            Self {
                postgresql,
                mysql,
                mariadb
            }
        }
    }
}