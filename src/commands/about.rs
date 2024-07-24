use crate::Error;
use crate::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This is a small test-bot! : )").await?;

    Ok(())
}
