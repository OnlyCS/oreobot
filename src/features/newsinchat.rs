use crate::prelude::*;

static mut WORKING_NEWS: Option<Shared<Option<serenity::Message>>> = None;

pub async fn register(ctx: &serenity::Context) {
    let data_arc = data::get_serenity(ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    unsafe { WORKING_NEWS = Some(Arc::new(Mutex::new(None))) }

    emitter.on_filter(
        events::MessageCreateEvent,
        |original, _| async move {
            let working_news = Arc::clone(unsafe { WORKING_NEWS.as_ref().unwrap() });
            let mut working_news = working_news.lock().await;
            *working_news = Some(original.clone());

            Ok(())
        },
        |message| {
            message.channel_id == nci::channels::NEWS
                && (message.content.contains("@everyone") || message.content.contains("@here"))
        },
    );

    emitter.on_filter(
        events::MessageCreateEvent,
        |clone, ctx| async move {
            let working_news = Arc::clone(unsafe { WORKING_NEWS.as_ref().unwrap() });
            let mut working_news = working_news.lock().await;
            let working_news = working_news.take();

            let Some(working_news) = working_news else {
                return Ok(());
            };

            let data_arc = data::get_serenity(&ctx).await;
            let mut data = data_arc.lock().await;
            let cache = &mut data.cache;

            cache
                .update::<cache_items::NewsInChat>(ctx, working_news, clone.id)
                .await?;

            Ok(())
        },
        |message| message.channel_id == nci::channels::CHAT && message.webhook_id.is_some(),
    )
}
