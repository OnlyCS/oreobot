pub mod emoji {
    pub const CURVED: &str = "1034653422416302151";
    pub const STRAIGHT: &str = "1034653871613681714";
    pub const PIN_EMOJI: &str = "📌";

    pub fn create<S0, S1>(id: S0, name: S1) -> String
    where
        S0: ToString,
        S1: ToString,
    {
        return format!("<:{}:{}>", id.to_string(), name.to_string());
    }
}

pub mod mention {
    pub enum MentionType {
        User,
        Role,
        Channel,
    }

    pub fn create<Id: ToString>(id: Id, kind: MentionType) -> String {
        match kind {
            MentionType::Role => {
                format!("<@&{}>", id.to_string())
            }
            MentionType::User => {
                format!("<@{}>", id.to_string())
            }
            MentionType::Channel => {
                format!("<#{}>", id.to_string())
            }
        }
    }
}

pub mod clone {
    use crate::prelude::*;

    pub async fn register(ctx: &serenity::Context) -> Result<()> {
        let prisma = prisma::create().await?;

        let to_resync = prisma
            .message_clone()
            .find_many(vec![message_clone::sync::equals(true)])
            .exec()
            .await?;

        let wh_id = ctx
            .cache
            .guild(nci::ID)
            .context("Could not find NCI")?
            .webhooks(&ctx)
            .await?
            .into_iter()
            .find(|n| {
                let Some(name) = n.name.as_ref() else {
                    return false;
                };

                name == "NCI Internals"
            })
            .context("Could not get webhook")?;

        let emitter_arc = Arc::clone(
            ctx.data
                .read()
                .await
                .get::<EventEmitterTypeKey>()
                .context("Could not find event emitter")?,
        );

        for clone in to_resync {
            clone_listen(
                Arc::clone(&emitter_arc),
                clone.cloned_message_id.parse()?,
                clone.cloned_from_id.parse()?,
                wh_id.id.0,
            )
            .await?;
        }

        Ok(())
    }

    pub async fn clone(
        ctx: &serenity::Context,
        message: &serenity::Message,
        link_to_prev: bool,
        clone_reply: bool,
        send_to: serenity::ChannelId,
        mut extra_rows: Vec<serenity::CreateActionRow>,
        sync: bool,
    ) -> Result<serenity::Message> {
        let emitter_arc = Arc::clone(
            ctx.data
                .read()
                .await
                .get::<EventEmitterTypeKey>()
                .context("Could not find event emitter")?,
        );

        let mut wh_message = serenity::ExecuteWebhook::default();
        let mut wh_row = serenity::CreateActionRow::default();
        let mut wh_content = "".to_string();

        if let Some(av) = message.author.avatar_url() {
            wh_message.avatar_url(av);
        }

        let username = {
            let attempt1 = message
                .member
                .as_ref()
                .map(|n| n.nick.as_ref())
                .flatten()
                .cloned();

            if let Some(name) = attempt1 {
                name
            } else {
                let attempt2 = message.author_nick(&ctx).await;

                if let Some(name) = attempt2 {
                    name
                } else {
                    let prisma = prisma::create().await?;

                    let user = prisma
                        .user()
                        .find_unique(user::id::equals(message.author.id.to_string()))
                        .exec()
                        .await?;

                    if let Some(user) = user.as_ref() {
                        user.nickname.as_ref().unwrap_or(&message.author.name)
                    } else {
                        &message.author.name
                    }
                    .clone()
                }
            }
        };

        wh_message.username(username);

        if link_to_prev {
            wh_row.create_button(|btn| {
                btn.style(serenity::ButtonStyle::Link);
                btn.label("Jump to original");
                btn.url(message.link());
                btn
            });
        }

        if clone_reply {
            // until rust-analyzer works with if-let chains, don't use them
            if let Some(reply) = message.referenced_message.as_ref() {
                let truncated = {
                    let mut content = reply.content.clone();

                    content.truncate(50);
                    content.push_str("...");

                    content
                };

                wh_content.push_str(&format!(
                    "{}\t{} {}\n{}\n",
                    emoji::create(emoji::CURVED, "curved"),
                    mention::create(reply.author.id, MentionType::User),
                    truncated,
                    emoji::create(emoji::STRAIGHT, "straight")
                ));

                wh_row.create_button(|btn| {
                    btn.style(serenity::ButtonStyle::Link);
                    btn.label("Jump to reply");
                    btn.url(reply.link());
                    btn
                });
            }
        }

        let mut components = serenity::CreateComponents::default();
        let mut all_rows = vec![];

        let embeds = &message.embeds;
        let attachments = &message.attachments;
        let content = &message.content;

        let wh_embeds = embeds
            .iter()
            .map(|embed| {
                serenity::Embed::fake(|create_embed| {
                    create_embed.clone_from(&embed.clone().into());
                    create_embed
                })
            })
            .collect::<Vec<_>>();

        wh_content.push_str(&content);
        wh_message.add_files(attachments.iter().map(|n| n.url.as_str()));
        wh_message.embeds(wh_embeds);
        wh_message.content(wh_content);

        all_rows.push(wh_row);
        all_rows.append(&mut extra_rows);
        components.set_action_rows(all_rows);
        wh_message.set_components(components);

        let maybe_webhook = send_to.webhooks(&ctx).await?.into_iter().find(|n| {
            let Some(name) = n.name.as_ref() else {
                return false;
            };

            name == "Oreo's Internals"
        });

        let webhook = if let Some(wh) = maybe_webhook {
            wh
        } else {
            send_to.create_webhook(&ctx, "Oreo's Internals").await?
        };

        let cloned = webhook
            .execute(&ctx, true, |exec| {
                exec.clone_from(&wh_message);
                exec
            })
            .await?
            .context("Could not get webhook message")?;

        if sync {
            let cloned_id = cloned.id.0;
            let wh_id = webhook.id.0;
            let from_id = message.id.0;

            clone_listen(emitter_arc, cloned_id, from_id, wh_id).await?;
        }

        let prisma = prisma::create().await?;

        prisma
            .message_clone()
            .create(
                cloned.id.to_string(),
                message::id::equals(message.id.to_string()),
                sync,
                vec![],
            )
            .exec()
            .await?;

        Ok(cloned)
    }

    async fn clone_listen(
        emitter_arc: Arc<Mutex<EventEmitter>>,
        wh_message_id: u64,
        original_id: u64,
        webhook_id: u64,
    ) -> Result<()> {
        let mut emitter = emitter_arc.lock().await;

        emitter.on_async_filter(
            events::MessageUpdateEvent,
            move |ev_message, ctx| async move {
                let webhook = ctx
                    .cache
                    .guild(nci::ID)
                    .context("Could not find guild")?
                    .webhooks(&ctx)
                    .await?
                    .into_iter()
                    .find(|wh| wh.id == webhook_id)
                    .context("Could not find webhook")?;

                let old_msg = webhook.get_message(&ctx, wh_message_id.into()).await?;

                webhook
                    .edit_message(&ctx, wh_message_id.into(), |wh_message| {
                        let embeds = &ev_message.embeds;
                        let content = &ev_message.content;

                        let reply_if_exists = {
                            let straight = emoji::create(emoji::STRAIGHT, "straight");

                            let old = old_msg.content;
                            let mut split = old.split(&straight);
                            let mut reply = None;

                            if let Some(first) = split.nth(0) {
                                if first.starts_with(&emoji::create(emoji::CURVED, "curved")) {
                                    let mut reply_string = first.to_string();

                                    reply_string.push_str(&straight);
                                    reply_string.push_str("\n\n");

                                    reply = Some(reply_string);
                                }
                            }

                            reply
                        };

                        if let Some(embeds) = embeds {
                            wh_message.embeds(
                                embeds
                                    .iter()
                                    .map(|embed| {
                                        serenity::Embed::fake(|create_embed| {
                                            create_embed.clone_from(&embed.clone().into());
                                            create_embed
                                        })
                                    })
                                    .collect::<Vec<_>>(),
                            );
                        }

                        if let Some(content) = content {
                            wh_message.content(format!(
                                "{}{}",
                                reply_if_exists.unwrap_or("".to_string()),
                                content
                            ));
                        }

                        wh_message
                    })
                    .await?;

                Ok(())
            },
            move |ev_message| ev_message.id.0 == original_id,
        );

        emitter.on_async_filter(
            events::MessageDeleteEvent,
            move |_, ctx| async move {
                ctx.cache
                    .guild(nci::ID)
                    .context("Could not find guild")?
                    .webhooks(&ctx)
                    .await?
                    .iter()
                    .find(|wh| wh.id.0 == webhook_id)
                    .context("Could not find webhook")?
                    .delete_message(&ctx, serenity::MessageId(wh_message_id))
                    .await?;

                Ok(())
            },
            move |payload| payload.message_id.0 == original_id,
        );

        Ok(())
    }
}