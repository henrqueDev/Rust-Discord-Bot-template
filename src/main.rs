use diesel_migrations::MigrationHarness;
extern crate dotenv;
use commands_list::commands;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenv::dotenv;

use poise::serenity_prelude as serenity;
use repository::connection_handler::get_connection;

pub mod commands;
pub mod commands_list;
pub mod util;
pub mod model;
pub mod repository;
pub mod schema;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub const MIGRATIONS : EmbeddedMigrations = embed_migrations!("./migrations");


#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut connection =  get_connection();
    
    connection
    .run_pending_migrations(MIGRATIONS)
    .expect("Error migrating");
    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

}
