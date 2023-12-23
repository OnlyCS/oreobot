use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MessageCloneOptions {
    pub update: bool,
    pub delete: bool,
    pub button: bool,
    pub member: Option<Member>,
    pub reason: MessageCloneReason,
}

impl Default for MessageCloneOptions {
    fn default() -> Self {
        Self {
            update: true,
            delete: true,
            button: true,
            member: None,
            reason: MessageCloneReason::Leak,
        }
    }
}

fn copy_user(
    wh: serenity::ExecuteWebhook,
    author: &Member,
) -> Result<serenity::ExecuteWebhook, serenity::Error> {
    Ok(wh.username(author.display_name()).avatar_url(author.face()))
}

fn create_reply(reply: &Message) -> Result<String, serenity::Error> {
    let truncated = reply
        .content
        .chars()
        .take(25)
        .chain("...".chars())
        .collect::<String>();

    let link = reply.link();

    let reply_text = format!(
        "{}{} [@{}: {truncated}]({link})\n",
        emoji::CURVED,
        emoji::STRAIGHT,
        reply.author.name,
    );

    Ok(reply_text)
}

async fn clone_attachments(
    ctx: &impl AsRef<serenity::Http>,
    attachments: &Vec<serenity::Attachment>,
) -> Vec<serenity::CreateAttachment> {
    stream::iter(attachments.iter())
        .filter_map(|attachment| async move {
            let url = &attachment.url;
            let create_attachment = serenity::CreateAttachment::url(&ctx, url).await.ok()?;
            Some(create_attachment)
        })
        .collect::<Vec<_>>()
        .await
}

fn clone_embeds(embeds: &Vec<serenity::Embed>) -> Vec<serenity::CreateEmbed> {
    embeds
        .iter()
        .cloned()
        .map(|emb| serenity::CreateEmbed::from(emb))
        .collect::<Vec<_>>()
}

async fn find_wh(
    ctx: &(impl serenity::CacheHttp + AsRef<serenity::Http>),
    channel: serenity::ChannelId,
) -> Result<serenity::Webhook, serenity::Error> {
    let webhooks = channel.webhooks(&ctx).await?;
    let webhook = webhooks.into_iter().find(|wh| {
        wh.application_id == ctx.http().application_id()
            && wh
                .name
                .as_ref()
                .is_some_and(|name| name == nci::webhook::NAME)
    });

    if let Some(webhook) = webhook {
        Ok(webhook)
    } else {
        channel
            .create_webhook(&ctx, serenity::CreateWebhook::new(nci::webhook::NAME))
            .await
    }
}

async fn construct_message(referenced_message: &Option<Box<Message>>, content: &String) -> String {
    let mut cloned_content = "".to_string();

    // build content
    referenced_message.as_ref().map(|reply| {
        cloned_content.push_str(&create_reply(reply).unwrap_or_else(|_| "".to_string()));
    });

    cloned_content.push_str(&content);

    cloned_content
}

async fn member_of(
    ctx: &impl serenity::CacheHttp,
    message: &Message,
) -> Result<Member, serenity::Error> {
    let member = message
        .member(&ctx)
        .await
        .unwrap_or(ctx.http().get_member(nci::ID, message.author.id).await?);

    Ok(member)
}

pub async fn message_clone(
    ctx: impl serenity::CacheHttp + AsRef<serenity::Http>,
    message: &Message,
    destination: ChannelId,
    options: MessageCloneOptions,
) -> Result<Message, MessageCloneError> {
    if !message.components.is_empty() {
        bail!(MessageCloneError::NoComponents)
    }

    let MessageCloneOptions {
        update,
        delete,
        button,
        member,
        reason,
    } = options;

    let member = member.unwrap_or(member_of(&ctx, &message).await?);

    let mut cloned_message = copy_user(serenity::ExecuteWebhook::new(), &member)?;

    // create jump buttons
    if button {
        cloned_message = cloned_message
            .button(serenity::CreateButton::new_link(message.link()).label("Original Message"));
    }

    let cloned_content = construct_message(&message.referenced_message, &message.content).await;
    let embeds = clone_embeds(&message.embeds);
    let attachments = clone_attachments(&ctx, &message.attachments).await;

    // build executor
    cloned_message = cloned_message
        .content(cloned_content)
        .embeds(embeds)
        .add_files(attachments);

    // find webhook
    let webhook = find_wh(&ctx, destination).await?;

    // send message
    let cloned_message = webhook.execute(&ctx, true, cloned_message).await?.unwrap();

    // database
    let mut client = Client::<LoggingServer>::new().await?;

    client
        .send(LoggingRequest::MessageCloneCreate {
            source: message.id,
            clone: cloned_message.id,
            destination,
            reason,
            update,
            update_delete: delete,
        })
        .await?;

    Ok(cloned_message)
}

pub async fn register() {
    mpmc::on(|ctx, event, _| async move {
        let mut logger = Client::<LoggingServer>::new().await?;

        let FullEvent::MessageUpdate { event: message, .. } = event else {
            bail!(EventError::UnwantedEvent);
        };

        let LoggingResponse::AllMessageClonesOk(list) =
            logger.send(LoggingRequest::MessageCloneReadAll).await?
        else {
            bail!(RouterError::<LoggingServer>::InvalidResponse)
        };

        let to_update = list
            .into_values()
            .filter(|n| n.source_id == i64::from(message.id))
            .filter(|n| n.update)
            .collect_vec();

        for update in to_update {
            let channel = ChannelId::new(update.destination_id as u64);
            let clone_id = MessageId::new(update.id as u64);
            let clone = channel.message(&ctx, clone_id).await?;
            let webhook = find_wh(&ctx, channel).await?;
            let had_button = !clone.components.is_empty();

            let cloned_content = construct_message(
                message.referenced_message.as_ref().unwrap(),
                message.content.as_ref().unwrap(),
            )
            .await;

            let embeds = clone_embeds(message.embeds.as_ref().unwrap());
            let mut edit = serenity::EditWebhookMessage::default()
                .content(cloned_content)
                .embeds(embeds);

            if had_button {
                edit = edit.components(vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new_link(
                        message.id.link(message.channel_id, message.guild_id),
                    )
                    .label("Original Message"),
                ])]);
            }

            webhook.edit_message(&ctx, clone_id, edit).await?;
        }

        Ok(())
    })
    .await;

    mpmc::on(|ctx, event, _| async move {
        let mut logger = Client::<LoggingServer>::new().await?;

        let FullEvent::MessageDelete {
            deleted_message_id: id,
            ..
        } = event
        else {
            bail!(EventError::UnwantedEvent);
        };

        let LoggingResponse::AllMessageClonesOk(list) =
            logger.send(LoggingRequest::MessageCloneReadAll).await?
        else {
            bail!(RouterError::<LoggingServer>::InvalidResponse)
        };

        let to_delete = list
            .into_values()
            .filter(|n| n.source_id == i64::from(id))
            .filter(|n| n.update_delete)
            .collect_vec();

        for delete in to_delete {
            let channel = ChannelId::new(delete.destination_id as u64);
            let clone_id = MessageId::new(delete.id as u64);
            let webhook = find_wh(&ctx, channel).await?;

            webhook.delete_message(&ctx, None, clone_id).await?;
        }

        Ok(())
    })
    .await;
}
