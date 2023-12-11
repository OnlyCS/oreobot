use crate::prelude::*;

async fn server(req: BotRequest, ctx: &serenity::Context) -> Result<BotResponse, BotError> {
    let response = match req {
        BotRequest::IsReady => BotResponse::Ready,
    };

    Ok(response)
}

pub async fn begin(ctx: serenity::Context) -> Result<!, RouterError<BotServer>> {
    let mut server = PersistServer::new(ctx, |req, ctx| {
        Box::pin(async move { server(req, ctx).await })
    })
    .await?;

    server.listen().await
}
