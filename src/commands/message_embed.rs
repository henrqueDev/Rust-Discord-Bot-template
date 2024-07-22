extern crate tracing;
use poise::CreateReply;
use ::serenity::all::CreateEmbed;
use ::serenity::all::CreateEmbedFooter;
use poise::serenity_prelude as serenity;


use crate::util::interaction::interaction_handler;
use crate::Error;
use crate::Context;


#[poise::command(slash_command, prefix_command)]
pub async fn message_embed(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://img.freepik.com/vetores-gratis/vetor-de-gradiente-de-logotipo-colorido-de-passaro_343694-1365.jpg?size=338&ext=jpg&ga=GA1.1.2008272138.1721520000&semt=ais_user";
    
    let footer = CreateEmbedFooter::new("This is a footer");

    let embed = CreateEmbed::new()
        .title("This is an embed")
        .description("With a description")
        .image(url)
        .footer(footer);

    let components = serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("verify")
            .label("Verificar")
            .style(serenity::ButtonStyle::Primary)
            .emoji('📋'),
    ]);

    let builder = CreateReply::default()
        .embed(embed.clone())
        .components(vec![components]);

    let reply = ctx.send(builder).await?;

    let start_time = std::time::Instant::now();

    let interaction = interaction_handler(ctx, reply, embed).await?;

    let pressed_button_id = match &interaction {
        Some(m) => &m.data.custom_id, // Get the component ID
        None => {
            ctx.say(":warning: You didn't interact in time - please run the command again.")
                .await?;
            return Ok(());
        }
    };

    let (register, global) = match &**pressed_button_id {
    "verify" => (true, true),
    other => {
        tracing::warn!("unknown register button ID: {:?}", other);
        return Ok(());
    }
    };

    if global {
        if register {
            ctx.say(format!(
                ":gear: Verifiying...",
            ))
            .await?;
        }
    }
    
    // Calulate time taken and send message
    let time_taken = start_time.elapsed();
    ctx.say(format!(
        ":white_check_mark: Done! Took {}ms",
        time_taken.as_millis()
    ))
    .await?;


    Ok(())
}