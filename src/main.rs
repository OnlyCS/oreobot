extern crate dotenv;
extern crate dotenv_codegen;
extern crate poise;
extern crate tokio;

mod commands;
mod env;
mod prelude;

use env::*;

#[tokio::main]
async fn main() {
    env_init();
    commands::run().await.unwrap();
}
