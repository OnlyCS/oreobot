use crate::prelude::*;

pub async fn register() {
    mpmc::on(|ctx, event, _| async move {
        let mut cache = Client::<CacheServer>::new().await?;

        let FullEvent::Message {
            new_message: message,
        } = event
        else {
            bail!(EventError::UnwantedEvent)
        };

        let CacheResponse::ImpersonationOk(Some(impersonation)) = cache
            .send(CacheRequest::GetImpersonation(message.author.id))
            .await?
        else {
            bail!(EventError::UnwantedEvent)
        };

        // at the same time == vroom
        let (clone, delete) = tokio::join!(
            clone::message_clone(
                &ctx,
                &message,
                message.channel_id,
                MessageCloneOptions {
                    reason: MessageCloneReason::Impersonation,
                    member: Some(ctx.http.get_member(nci::ID, impersonation).await?),
                    button: false,
                    update: false,
                    delete: false,
                },
            ),
            message.delete(&ctx)
        );

        clone?;
        delete?;

        Ok(())
    })
    .await;
}
