use crate::prelude::*;

pub async fn send_ephemeral<'a>(
    ctx: impl serenity::CacheHttp,
    user: serenity::UserId,
    f: impl for<'b> FnOnce(&'b mut serenity::CreateMessage<'a>) -> &'b mut serenity::CreateMessage<'a>,
) -> Result<(), AnyError> {
    ctx.cache()
        .unwrap()
        .user(user)
        .unwrap()
        .direct_message(&ctx, f)
        .await?;

    Ok(())
}
