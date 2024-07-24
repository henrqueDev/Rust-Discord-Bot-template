use poise::CreateReply;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn servers_list(ctx: Context<'_>) -> Result<(), Error> {


    let show_private_guilds = ctx.framework().options().owners.contains(&ctx.author().id);

    // Aggregate all guilds and sort them by size
    let mut hidden_guilds = 0;
    let mut shown_guilds = Vec::<(String, u64)>::new();
    for guild_id in ctx.cache().guilds() {
        println!("{:#?}", guild_id);
        match ctx.cache().guild(guild_id) {
            Some(guild) => {
                let is_public = guild.features.iter().any(|x| x == "DISCOVERABLE");
                if !is_public && !show_private_guilds {
                    hidden_guilds += 1; // private guild whose name and size shouldn't be exposed
                } else {
                    shown_guilds.push((guild.name.clone(), guild.member_count))
                }
            }
            None => hidden_guilds += 1, // uncached guild
        }
    }
    shown_guilds.sort_by_key(|(_, member)| u64::MAX - member); // sort largest guilds first

    // Iterate guilds and build up the response message line by line
    let mut response = format!(
        "I am currently in {} servers!\n",
        shown_guilds.len() + hidden_guilds
    );
    if show_private_guilds {
        response.insert_str(0, "_Showing private guilds because you are a bot owner_\n");
    }

    let reply = CreateReply::default()
        .content(response);

    ctx.send(reply).await?;

    Ok(())
}