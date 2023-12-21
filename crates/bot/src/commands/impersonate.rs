use crate::prelude::*;

#[poise::command(slash_command, subcommands("set", "unset"))]
pub async fn impersonate(_: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

#[poise::command(slash_command)]
async fn set(ctx: Context<'_>, to: serenity::Member) -> Result<(), CommandError> {
    let mut cache = Client::<CacheServer>::new().await?;

    if to.user.bot {
        bail!(CommandError::IllegalArgument(String::from(
            "Cannot impersonate a bot"
        )));
    }

    if to.user.id == ctx.author().id {
        bail!(CommandError::IllegalArgument(String::from(
            "Cannot impersonate yourself"
        )));
    }

    cache
        .send(CacheRequest::SetImpersonation(ctx.author().id, to.user.id))
        .await?;

    let embed = embed::default(EmbedStatus::Success)
        .title("Impersonation > Set")
        .description(format!(
            "Successfully set impersonation to {}",
            mention::create(to.user.id, MentionType::User)
        ));

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn unset(ctx: Context<'_>) -> Result<(), CommandError> {
    let mut cache = Client::<CacheServer>::new().await?;

    cache
        .send(CacheRequest::StopImpersonation(ctx.author().id))
        .await?;

    let embed = embed::default(EmbedStatus::Success)
        .title("Impersonation > Unset")
        .description("Successfully unset impersonation");

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}
