pub mod env;


pub fn load_all_env() {
    env::general::load();
    env::database::mysql::load();
    env::database::posgresql::load();

    println!("{:?}", env::general::api_key());
    println!("{:?}", env::general::port());
    println!("{:?}", env::database::mysql::db());
    println!("{:?}", env::database::posgresql::db());
}
