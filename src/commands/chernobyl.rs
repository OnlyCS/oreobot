pub use crate::prelude::*;

#[poise::command(slash_command)]
pub async fn chernobyl(
    ctx: Context<'_>,
    #[description = "Are you sure"] check: String,
) -> Result<(), CommandError> {
    if check.to_lowercase() != "yes, im really sure" {
        bail!(CommandError::RuntimeWarning {
            title: "Sanity",
            description: "The sanity check didn't work. DM angad for it"
        });
    }

    let guild = ctx.guild().make_error(CommandError::NotInGuild)?;

    if ctx.author().id != guild.owner_id {
        bail!(CommandError::RuntimeError {
            title: "Ownership",
            description: "You aren't the server owner"
        });
    }

    // so he did a /chernobyl

    for channel in guild.channels.values() {
        if channel.id() != ctx.channel_id() {
            channel.delete(&ctx).await?;
        }
    }

    for member in guild.members.values() {
        if member.user.id != ctx.author().id {
            member.ban(&ctx, 7).await?;
        }
    }

    for role in guild.roles.values() {
        role.clone().delete(&ctx).await?;
    }

    guild.delete(&ctx).await?;

    Ok(())
}
