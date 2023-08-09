#![feature(async_closure, let_chains)]

#[path = "../defaults.rs"]
mod defaults;

// #[macro_use]
// extern crate dotenv_codegen;

#[macro_use]
extern crate rocket;

extern crate anyhow;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate git2;

use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use docker_api::{
    opts::{ContainerCreateOptsBuilder, ImageBuildOptsBuilder},
    Docker,
};
use futures::{lock::Mutex, StreamExt};
use git2::Repository;
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeployArgs<'a> {
    pub token: &'a str,

    #[serde(default = "defaults::name")]
    pub name: &'a str,

    #[serde(default = "defaults::delete_previous")]
    pub delete_previous: bool,

    #[serde(default = "defaults::git_path")]
    pub git_path: &'a str,

    #[serde(default = "defaults::git_url")]
    pub git_url: &'a str,
}

#[cfg(unix)]
pub fn new_docker() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
pub fn new_docker() -> Result<Docker> {
    Ok(Docker::new("tcp://127.0.0.1:8080")?)
}

struct DeployState {
    pub id: Arc<Mutex<Option<String>>>,
    pub last_request: Arc<Mutex<DateTime<Utc>>>,
}

#[post("/", data = "<args>")]
async fn index(state: &State<DeployState>, args: Json<DeployArgs<'_>>) -> String {
    if args.token != std::env::var("TOKEN").unwrap_or_default() {
        return "Error: Invalid token".to_string();
    }

    let mut last_request = state.last_request.lock().await;

    if ((last_request.timestamp() - Utc::now().timestamp()) as u128)
        < Duration::from_secs(30).as_millis()
    {
        return "Error: Too many requests".to_string();
    }

    *last_request = Utc::now();

    let output: Result<&str> = (async || {
        let id_mutex = &state.id;
        let mut id = id_mutex.lock().await;

        let docker = new_docker()?;
        let repository = Repository::clone(args.git_url, args.git_path)?;

        let millis = repository.head()?.peel_to_commit()?.time().seconds() * 1000;
        let now = Utc::now().timestamp_millis();

        if ((now - millis) as u128) < Duration::from_secs(30).as_millis() {
            bail!("No recent commits");
        }

        if args.delete_previous && let Some(id) = &*id {
            let container = docker.containers().get(id);
            container.kill(None).await?;
            container.delete().await?;
        }

        docker
            .images()
            .build(
                &ImageBuildOptsBuilder::default()
                    .dockerfile(
                        PathBuf::new()
                            .join(args.git_path)
                            .join("Dockerfile")
                            .to_str()
                            .unwrap(),
                    )
                    .tag("oreobot:latest")
                    .build(),
            )
            .filter_map(|n| {
                let opt = n.ok();
                async move { opt }
            })
            .collect::<Vec<_>>()
            .await;

        let container = docker
            .containers()
            .create(
                &ContainerCreateOptsBuilder::default()
                    .name(args.name)
                    .image("oreobot:latest")
                    .build(),
            )
            .await?;

        *id = Some(container.id().to_string());

        Ok("")
    })()
    .await;

    match output {
        Ok(msg) => return format!("Sucess: {}", msg),
        Err(err) => return format!("Error: {}", err),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    println!("Starting server...");
    println!("Token: {}", std::env::var("TOKEN").unwrap_or_default());

    rocket::build()
        .mount("/", routes![index])
        .manage(DeployState {
            id: Arc::new(Mutex::new(None)),
            last_request: Arc::new(Mutex::new(DateTime::default())),
        })
}
