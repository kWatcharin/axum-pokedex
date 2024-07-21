pub mod env;

pub fn load_all_env() {
    env::main::load();
    env::database::mysql::load();
    env::database::posgresql::load();
    env::database::mariadb::load();
}
