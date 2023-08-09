extern crate anyhow;
extern crate dotenv;
extern crate reqwest;
extern crate serde_json;
extern crate tokio;

use anyhow::{bail, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let token = std::env::var("TOKEN")?;

    reqwest::Client::new()
        .post(dotenv::var("SERVER")?)
        .json(&json!({
            "token": token,
        }))
        .send()
        .await?;

    Ok(())
}
