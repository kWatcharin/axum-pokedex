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
            .expect("⚠️  PORT in .env must be set!")
            .parse::<u16>()
            .expect("⚠️  PORT in .env must be number!");

        #[derive(Debug)]
        pub static ref TIME_ZONE: String = env::var("TIME_ZONE")
            .expect("⚠️  TIME_ZONE in .env must be set!");

        #[derive(Debug)]
        pub static ref SECRET_KEY: String = env::var("API_KEY")
            .expect("⚠️  API_KEY in .env must be set!");

        #[derive(Debug)]
        pub static ref API_KEY: String = env::var("SECRET_KEY")
            .expect("⚠️  SECRET_KEY in .env must be set!");

        #[derive(Debug)]
        pub static ref SERVICE_ACCOUNT: String = env::var("SERVICE_ACCOUNT")
            .expect("⚠️  SERVICE_ACCOOUNT in .env must be set!");

        #[derive(Debug)]
        pub static ref CLIENT_ID: u8 = env::var("CLIENT_ID")
            .expect("⚠️  CLIENT_ID in .env must be set!")
            .parse::<u8>()
            .expect("⚠️  CLIENT_ID in .env must be number!");

        #[derive(Debug)]
        pub static ref CLIENT_SECRET: String = env::var("CLIENT_SECRET")
            .expect("⚠️  CLIENT_SECRET in .env must be set!");
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
                .expect("⚠️  MYSQL_DB in .mysql.env must be set!");
        }
    }

    pub mod posgresql {
        use super::*;

        pub fn load() {
            from_path("env/.postgresql.env").ok();
        }

        lazy_static! {
            #[derive(Debug)]
            pub static ref HOST: String = env::var("POSTGRES_HOST")
                .expect("⚠️  POSTGRES_HOST in .postgresql.env must be set!");

            #[derive(Debug)]
            pub static ref PORT: u16 = env::var("POSTGRES_PORT")
                .expect("⚠️  POSTGRES_PORT in .postgresql.env must be set!")
                .parse::<u16>()
                .expect("⚠️  POSTGRES_PORT in .postgresql.env must be number!");

            #[derive(Debug)]
            pub static ref DB: String = env::var("POSTGRES_DB")
                .expect("⚠️  POSTGRES_DB in .postgresql.env must be set!");

            #[derive(Debug)]
            pub static ref USER: String = env::var("POSTGRES_USER")
                .expect("⚠️  POSTGRES_USER in .postgresql.env must be set!");

            #[derive(Debug)]
            pub static ref PASSWORD: String = env::var("POSTGRES_PASSWORD")
                .expect("⚠️  POSTGRES_PASSWORD in .postgresql.env must be set!");

            #[derive(Debug)]
            pub static ref URL: String = env::var("POSTGRES_URL")
                .expect("⚠️  POSTGRES_URL in .postgresql.env must be set!");

            #[derive(Debug)]
            pub static ref MAX_CONNECTION: u32 = env::var("POSTGRES_MAX_CONNECTION")
                .expect("⚠️  POSTGRES_MAX_CONNECTION in .postgresql.env must be set!")
                .parse::<u32>()
                .expect("⚠️  POSTGRES_MAX_CONNECTION in .postgresql.env must be number!");

            #[derive(Debug)]
            pub static ref TIMEOUT: u64 = env::var("POSTGRES_TIMEOUT")
                .expect("⚠️  POSTGRES_TIMEOUT in .postgresql.env must be set!")
                .parse::<u64>()
                .expect("⚠️  POSTGRES_TIMEOUT must be number!");
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
                .expect("⚠️  MARIADB_DB .mariadb.env must be set!");

            #[derive(Debug)]
            pub static ref HOST: String = env::var("MARIADB_HOST")
                .expect("⚠️  MARIADB_HOST .mariadb.env must be set!");

            #[derive(Debug)]
            pub static ref PORT: u16 = env::var("MARIADB_PORT")
                .expect("⚠️  MARIADB_PORT .mariadb.env must be set!")
                .parse::<u16>()
                .expect("⚠️  MARIADB_PORT .mariadb.env must be number!");

            #[derive(Debug)]
            pub static ref USERNAME: String = env::var("MARIADB_USERNAME")
                .expect("⚠️  MARIADB_USERNAME .mariadb.env must be set!");

            #[derive(Debug)]
            pub static ref PASSWORD: String = env::var("MARIADB_PASSWORD")
                .expect("⚠️  MARIADB_PASSWORD .mariadb.env must be set!");
        }
    }
}