use events::clone::CloneArgsBuilder;

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
            .get::<cache::impersonate::Impersonation>()
            .await?
            .get(&author_id)
            .cloned()
            .flatten();

        let Some(clone_as) = user else { return Ok(()) };
        message.delete(&ctx).await?;

        let cloned = clone::clone(CloneArgsBuilder::build_from(|args| {
            args.ctx(ctx.clone());
            args.message(message.clone());
            args.jump_btn(false);
            args.clone_as(ctx.cache.user(clone_as).unwrap());
            args.destination(message.channel_id);
            args.ping(true);
        })?)
        .await?;

        let prisma = prisma::create().await?;

        prisma
            .message()
            .create(
                message.id.to_string(),
                message.content,
                user::id::equals(message.author.id.to_string()),
                channel::id::equals(message.channel_id.to_string()),
                vec![],
            )
            .exec()
            .await?;

        for attachment in message.attachments {
            prisma
                .attachment()
                .create(
                    attachment.id.to_string(),
                    attachment.filename,
                    attachment.url,
                    attachment.size as i64, //cannot exceed 100gb, so i64 is fine
                    message::id::equals(message.id.to_string()),
                    vec![],
                )
                .exec()
                .await?;
        }

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

        Ok(())
    });
}
