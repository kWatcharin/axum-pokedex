use std::env;
use dotenv::from_path;


pub mod general {
    use super::*;

    pub fn load() {
        from_path("env/.env").ok();
    }

    pub fn port() -> u16 {
        env::var("PORT")
            .expect("PORT must be set!")
            .parse::<u16>()
            .expect("PORT must be number!")
    }

    type PortResult<T, E = std::env::VarError> = core::result::Result<T, E>;
    #[allow(unused)]
    pub fn port_v2() -> PortResult<String> {
        Ok(
            env::var("PORT")?
        )
    }

    #[allow(unused)]
    pub fn api_key() -> String {
        env::var("API_KEY")
            .expect("API_KEY must be set!")
    }

    #[allow(unused)]
    pub fn secret_key() -> String {
        env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set!")
    }
}


pub mod database {
    use super::*;

    pub mod mysql {
        use super::*;

        pub fn load() {
            from_path("env/.mysql.env").ok();
        }

        #[allow(unused)]
        pub fn db() -> String {
            env::var("MYSQL_DB").expect("MYSQL_DB must be set!")
        }
    }

    pub mod posgresql {
        use super::*;

        pub fn load() {
            from_path("env/.postgresql.env").ok();
        }

        #[allow(unused)]
        pub fn db() -> String {
            env::var("POSTGRESQL_DB").expect("POSTGRESQL_DB must be set!")
        }
    }
}
