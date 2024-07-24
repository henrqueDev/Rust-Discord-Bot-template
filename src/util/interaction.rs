use poise::CreateReply;
use poise::ReplyHandle;
use serenity::all::ComponentInteraction;
use serenity::all::CreateEmbed;


use crate::Context;
use crate::Error;

pub async fn interaction_handler(ctx: Context<'_>, reply: ReplyHandle<'_>, embed: CreateEmbed) -> Result<Option<ComponentInteraction>, Error> {

    let interaction = reply
    .message()
    .await?
    .await_component_interaction(ctx)
    .author_id(ctx.author().id)
    .await;

    reply
    .edit(
        ctx,
        CreateReply::default()
            .content("")
            .embed(embed)
            .components(vec![])
    ).await?; // remove buttons after button press and edit message

    

    Ok(interaction)
}