use crate::prelude::*;

async fn _send_ephemeral<'a, R>(
    ctx: impl AsRef<serenity::Http>,
    token: &str,
    f: impl for<'b> FnOnce(&'b mut serenity::CreateInteractionResponseFollowup<'a>) -> R,
) -> Result<(), serenity::Error> {
    let mut response = serenity::CreateInteractionResponseFollowup::default();
    f(&mut response);

    let http = ctx.as_ref();

    let map = response
        .0
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<serde_json::Map<_, _>>();

    http.create_followup_message(token, &serde_json::Value::from(map))
        .await?;

    Ok(())
}

pub async fn send_ephemeral<'a, R>(
    ctx: impl AsRef<serenity::Http>,
    user: serenity::UserId,
    f: impl for<'b> FnOnce(&'b mut serenity::CreateInteractionResponseFollowup<'a>) -> R,
    to: serenity::ChannelId,
) -> Result<(), AnyError> {
    let info = ctx.as_ref().get_current_application_info().await?;
    let app_id = info.id;
    let prisma = prisma::create().await?;

    let token = prisma
        .interaction()
        .find_first(vec![and![
            interaction::invoker_id::equals(user.to_string()),
            interaction::application_id::equals(app_id.to_string()),
            interaction::channel_id::equals(to.to_string()),
            interaction::reusable::equals(true)
        ]])
        .exec()
        .await?
        .make_error(anyhow!("This user has no interaction"))?
        .token;

    _send_ephemeral(ctx, &token, f).await?;

    Ok(())
}
