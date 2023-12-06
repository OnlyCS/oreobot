use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MessageCloneOptions {
    pub update: bool,
    pub delete: bool,
    pub button: bool,
    pub member: Option<serenity::Member>,
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
    author: &serenity::Member,
) -> Result<serenity::ExecuteWebhook, serenity::Error> {
    Ok(wh.username(author.display_name()).avatar_url(author.face()))
}

fn create_reply(reply: &serenity::Message) -> Result<String, serenity::Error> {
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
    message: &serenity::Message,
) -> Vec<serenity::CreateAttachment> {
    stream::iter(message.attachments.iter())
        .filter_map(|attachment| async move {
            let url = &attachment.url;
            let create_attachment = serenity::CreateAttachment::url(&ctx, url).await.ok()?;
            Some(create_attachment)
        })
        .collect::<Vec<_>>()
        .await
}

fn clone_embeds(message: &serenity::Message) -> Vec<serenity::CreateEmbed> {
    message
        .embeds
        .iter()
        .cloned()
        .map(|emb| serenity::CreateEmbed::from(emb))
        .collect::<Vec<_>>()
}

async fn find_wh(
    ctx: &(impl serenity::CacheHttp + AsRef<serenity::Http>),
    channel: &serenity::ChannelId,
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

pub async fn message_clone(
    ctx: &(impl serenity::CacheHttp + AsRef<serenity::Http>),
    message: &serenity::Message,
    destination: &serenity::ChannelId,
    options: MessageCloneOptions,
) -> Result<serenity::Message, CloneError> {
    if !message.components.is_empty() {
        bail!(CloneError::NoComponents)
    }

    let MessageCloneOptions {
        update,
        delete,
        button,
        member,
        reason,
    } = options;

    let member = member.unwrap_or(message.member(&ctx).await?);
    let mut cloned_message = copy_user(serenity::ExecuteWebhook::new(), &member)?;
    let mut cloned_content = "".to_string();

    // create jump buttons
    if button {
        cloned_message = cloned_message
            .button(serenity::CreateButton::new_link(message.link()).label("Original Message"));
    }

    // build content
    message.referenced_message.as_ref().map(|reply| {
        cloned_content.push_str(&create_reply(reply).unwrap_or_else(|_| "".to_string()));
    });

    cloned_content.push_str(&message.content);

    // create attachments
    let attachments = clone_attachments(&ctx, &message).await;

    // create embeds
    let embeds = clone_embeds(&message);

    // build executor
    cloned_message = cloned_message
        .content(cloned_content)
        .embeds(embeds)
        .add_files(attachments);

    // find webhook
    let webhook = find_wh(&ctx, &destination).await?;

    // send message
    let cloned_message = webhook.execute(&ctx, true, cloned_message).await?.unwrap();

    // database
    let mut client = Client::<LoggingServer>::new().await?;

    client
        .send(LoggingRequest::MessageCloneCreate {
            source: message.id,
            clone: cloned_message.id,
            destination: *destination,
            reason,
            update,
            update_delete: delete,
        })
        .await?;

    Ok(cloned_message)
}
