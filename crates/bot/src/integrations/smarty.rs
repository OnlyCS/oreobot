use crate::prelude::*;

lazy_static::lazy_static! {
    static ref NEWS_MESSAGE: Arc<Mutex<Option<Message>>> = Arc::new(Mutex::new(None));
}

pub async fn register() {
    mpmc::on(|ctx, event, _| async move {
        let FullEvent::Message {
            new_message: message,
        } = event
        else {
            bail!(EventError::UnwantedEvent)
        };

        if message.channel_id != nci::channels::NEWS {
            bail!(EventError::UnwantedEvent)
        }

        if !check_smarty(&ctx).await {
            news_clone::perform(ctx, message).await?;

            return Ok(());
        }

        let mut news_message = NEWS_MESSAGE.lock().await;
        *news_message = Some(message);

        Ok(())
    })
    .await;

    mpmc::on(|_, event, _| async move {
        let FullEvent::Message {
            new_message: message,
        } = event
        else {
            bail!(EventError::UnwantedEvent)
        };

        if message.channel_id != nci::channels::CHAT
            && message
                .webhook_id
                .is_some_and(|id| id == nci::smarty::WEBHOOK_CHAT)
        {
            bail!(EventError::UnwantedEvent)
        }

        // we have a complete message clone
        let mut news_message = NEWS_MESSAGE.lock().await;
        let news_message = news_message.take().unwrap();

        // register the clone
        let mut logger = Client::<LoggingServer>::new().await?;

        logger
            .send(LoggingRequest::MessageCloneCreate {
                source: news_message.id,
                clone: message.id,
                destination: nci::channels::CHAT,
                reason: MessageCloneReason::NewsInChat,
                // we don't own the webhook, we are integrating
                update: false,
                update_delete: false,
            })
            .await?;

        Ok(())
    })
    .await;
}

pub async fn check_smarty(ctx: &serenity::Context) -> bool {
    let status = ctx
        .cache
        .guild(nci::ID)
        .map(|guild| guild.presences.get(&nci::smarty::ID).cloned())
        .flatten()
        .unwrap();

    status.status == serenity::OnlineStatus::Online
}
