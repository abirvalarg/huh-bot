mod handler;
use serenity::Client;
/*
use std::{
    sync::Arc,
    collections::HashMap
};
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{
        TypeMapKey,
        Client
    },
    framework::standard::{
        macros::{
            check,
            command,
            group,
            help,
            hook
        }
    }
};
use tokio::sync::Mutex;


struct SMContainer;

impl TypeMapKey for SMContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[group]
#[commands(ping)]
struct General;
*/

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN")?;
    let mut client = Client::builder(&token).event_handler(handler::Handler).await?;
    client.start().await?;
    Ok(())
}
