use crate::prelude::*;

pub async fn register(ctx: &serenity::Context) {
    let data_arc = data::get_serenity(ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    emitter.on(events::MessageCreateEvent, |message, ctx| async move {
        let author_id = message.author.id;

        let data_arc = data::get_serenity(&ctx).await;
        let mut data = data_arc.lock().await;
        let cache = &mut data.cache;

        let user = cache
            .get_user::<cache::impersonate::Impersonation>(author_id)
            .await?;

        let Some(clone_as) = user else { return Ok(()) };

        let cloned = clone::clone(
            &ctx,
            &message,
            false,
            true,
            message.channel_id,
            vec![],
            false,
            ctx.cache.user(clone_as),
        )
        .await?;

        let prisma = prisma::create().await?;

        prisma
            .impersonated_message_data()
            .create(
                message::id::equals(message.id.to_string()),
                user::id::equals(clone_as.to_string()),
                cloned.id.to_string(),
                vec![],
            )
            .exec()
            .await?;

        message.delete(&ctx).await?;

        Ok(())
    });
}
