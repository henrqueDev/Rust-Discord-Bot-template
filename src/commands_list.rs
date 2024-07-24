use crate::{commands::{about::about, message_embed::message_embed, ping::ping, servers_list::servers_list}, Data};

pub fn commands() -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    return vec![
        ping(),
        about(),
        message_embed(),
        servers_list()
    ]
}