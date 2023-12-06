use crate::prelude::*;

use super::MpmcData;

async fn _handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _fw_ctx: FrameworkContext<'_>,
    data: &Data,
) -> Result<(), CommandError> {
    super::send(MpmcData {
        event: event.clone(),
        ctx: ctx.clone(),
        data: Arc::clone(data),
    })
    .await?;

    Ok(())
}

pub fn handler<'a>(
    ctx: &'a serenity::Context,
    event: &'a serenity::FullEvent,
    fw_ctx: FrameworkContext<'a>,
    data: &'a Data,
) -> BoxFuture<'a, Result<(), CommandError>> {
    Box::pin(async move { _handler(ctx, event, fw_ctx, data).await })
}
