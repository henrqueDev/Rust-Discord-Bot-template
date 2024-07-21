use serenity::all::{Context, Message};
use crate::commands::ping::ping;

pub async fn msg_handler(ctx: Context, msg: Message) {
    let message = msg.content.as_str();
    
    match message {
            "!ping" => ping(ctx, msg).await,
            _ => println!("")    
    }
}