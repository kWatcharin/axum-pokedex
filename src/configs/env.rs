use std::env;
use dotenv::from_path;


pub mod general {
    use super::*;

    pub fn load() {
        from_path("env/.env").ok();
    }

    #[allow(unused)]
    pub fn api_key() -> String {
        env::var("API_KEY")
            .expect("API_KEY must be set!")
    }

    pub fn port() -> u16 {
        env::var("PORT")
            .expect("PORT must be set!")
            .parse::<u16>()
            .expect("PORT must be number!")
    }
}


pub mod database {
    use super::*;

    pub mod mysql {
        use super::*;

        pub fn load() {
            from_path("env/.mysql.env").ok();
        }

        pub fn db() -> String {
            env::var("MYSQL_DB").expect("MYSQL_DB must be set!")
        }
    }

    pub mod posgresql {
        use super::*;

        pub fn load() {
            from_path("env/.postgresql.env").ok();
        }

        pub fn db() -> String {
            env::var("POSTGRESQL_DB").expect("POSTGRESQL_DB must be set!")
        }
    }
}
