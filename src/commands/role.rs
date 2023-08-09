use crate::prelude::*;

#[poise::command(slash_command, subcommands("name", "color"))]
pub async fn role(_: Context<'_>) -> Result<()> {
    Ok(())
}

async fn get_role_id(ctx: &Context<'_>, member: Option<serenity::Member>) -> Result<String> {
    get_prisma::from_poise_context!(prisma, ctx);

    let member_id = if let Some(member) = member {
        if !prisma
            .user()
            .find_unique(user::id::equals(member.user.id.to_string()))
            .exec()
            .await?
            .context("Could not find user")?
            .admin
        {
            bail!("You must be an admin to manage other users or bots");
        }

        member.user.id.to_string()
    } else {
        ctx.author().id.to_string()
    };

    let roles = prisma
        .role()
        .find_first(vec![
            role::color_role::equals(true),
            role::color_role_user::is(vec![user::id::equals(member_id)]),
        ])
        .exec()
        .await?;

    match roles {
        Some(n) => Ok(n.id),
        None => {
            logging::ready::on_ready(ctx.serenity_context().clone()).await?;
            bail!("Warning: The user or role could not be found. The database is being reindexed. Please try again shortly.");
        }
    }
}

#[poise::command(slash_command)]
async fn name(
    ctx: Context<'_>,
    #[description = "The name to change the role to"] name: String,
    #[description = "The user's role to change -- admin only!"] member: Option<serenity::Member>,
) -> Result<()> {
    let role_id = get_role_id(&ctx, member).await?;

    ctx.guild()
        .context("This command can only be used in a guild")?
        .edit_role(&ctx, role_id.parse::<u64>()?, |role| {
            role.name(name.clone());
            role
        })
        .await?;

    ctx.send(|reply| {
        let mut embed = embed::default(&ctx, embed::EmbedStatus::Sucess);

        embed.title("Role - Name");
        embed.description(format!("Role name sucessfully changed to `{}`", name));

        reply.embeds.push(embed);
        reply.ephemeral(true);

        reply.components(|comp| comp.add_action_row(share::row(false)));

        reply
    })
    .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn color(
    ctx: Context<'_>,
    #[description = "The color to set. Accepts (r,g,b) and #hex. No words accepted"] color: String,
    #[description = "The user's role to change -- admin only!"] member: Option<serenity::Member>,
) -> Result<()> {
    // attempt to parse the color

    let role_id = get_role_id(&ctx, member).await?;

    let color_struct = {
        if color.starts_with("(") {
            let color = color.trim_start_matches("(").trim_end_matches(")");
            let number_strs = color.split(",");

            let numbers = number_strs
                .map(|n| n.trim())
                .map(|n| n.parse::<u8>())
                .collect::<Vec<_>>();

            if numbers.len() != 3 {
                bail!("Invalid color format");
            }

            serenity::Color::from((
                numbers[0].clone()?,
                numbers[1].clone()?,
                numbers[2].clone()?,
            ))
        } else if color.starts_with("#") {
            colors::hex_to_color(color.trim_start_matches("#"))?
        } else {
            bail!("Invalid color format");
        }
    };

    ctx.guild()
        .context("This command can only be used in a guild")?
        .edit_role(&ctx, role_id.parse::<u64>()?, |role| {
            role.colour(color_struct.0.into());
            role
        })
        .await?;

    ctx.send(|reply| {
        let mut embed = embed::default(&ctx, embed::EmbedStatus::Sucess);

        embed.title("Role - Color");
        embed.description(format!("Role color sucessfully changed to `{}`", color));

        reply.embeds.push(embed);
        reply.ephemeral(true);

        reply.components(|comp| comp.add_action_row(share::row(false)));

        reply
    })
    .await?;

    Ok(())
}
