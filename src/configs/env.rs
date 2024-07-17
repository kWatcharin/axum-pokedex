use std::env;
use dotenv::from_path;
use lazy_static::lazy_static;


pub mod main {
    use super::*;

    pub fn load() {
        from_path("env/.env").ok();
    } 
    
    lazy_static! {
        #[derive(Debug)]
        pub static ref PORT: u16 = env::var("PORT")
            .expect("PORT must be set!")
            .parse::<u16>()
            .expect("PORT must be number!");

        #[derive(Debug)]
        pub static ref TIME_ZONE: String = env::var("TIME_ZONE")
            .expect("TIME_ZONE must be set!");

        #[derive(Debug)]
        pub static ref SECRET_KEY: String = env::var("API_KEY")
            .expect("API_KEY must be set!");

        #[derive(Debug)]
        pub static ref API_KEY: String = env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set!");
    }
}


pub mod database {
    use super::*;

    pub mod mysql {
        use super::*;

        pub fn load() {
            from_path("env/.mysql.env").ok();
        }

        lazy_static! {
            #[derive(Debug)]
            pub static ref DB: String = env::var("MYSQL_DB")
                .expect("MYSQL_DB must be set!");
        }
    }

    pub mod posgresql {
        use super::*;

        pub fn load() {
            from_path("env/.postgresql.env").ok();
        }

        lazy_static! {
            #[derive(Debug)]
            pub static ref DB: String = env::var("POSTGRESQL_DB")
                .expect("POSTGRESQL_DB must be set!");
        }
    }

    pub mod mariadb {
        use super::*;

        pub fn load() {
            from_path("env/.mariadb.env").ok();
        }

        lazy_static! {
            #[derive(Debug)]
            pub static ref DB: String = env::var("MARIADB_DB")
                .expect("MARIADB_DB must be set!");

            #[derive(Debug)]
            pub static ref HOST: String = env::var("MARIADB_HOST")
                .expect("MARIADB_HOST must be set!");

            #[derive(Debug)]
            pub static ref PORT: u16 = env::var("MARIADB_PORT")
                .expect("MARIADB_PORT must be set!")
                .parse::<u16>()
                .expect("MARIADB_PORT must be number!");

            #[derive(Debug)]
            pub static ref USERNAME: String = env::var("MARIADB_USERNAME")
                .expect("MARIADB_USERNAME must be set!");

            #[derive(Debug)]
            pub static ref PASSWORD: String = env::var("MARIADB_PASSWORD")
                .expect("MARIADB_PASSWORD must be set!");
        }
    }
}