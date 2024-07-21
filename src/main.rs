extern crate dotenv;
use std::{env, sync::Arc};

use dotenv::dotenv;
use handler::handler_model::Handler;
use serenity::prelude::*;

pub mod handler;
pub mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    // Create a new instance of the Client, logging in as a bot.
    let event_handler_arc = Arc::new(Handler);
    let mut client =
        Client::builder(&token, intents).event_handler_arc(event_handler_arc).await.expect("Err creating client");
    
    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
