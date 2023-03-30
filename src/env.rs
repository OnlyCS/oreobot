use dotenv::dotenv;

pub fn env_init() {
    dotenv().ok();
}
