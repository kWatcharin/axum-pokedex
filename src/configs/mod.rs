pub mod env;


pub fn load_all_env() {
    env::general::load();
    env::database::mysql::load();
    env::database::posgresql::load();
}
