use serenity::{all::{Context, EventHandler, Message}, async_trait};

use super::message::message_handler::msg_handler;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        msg_handler(ctx, msg).await;
    }
}