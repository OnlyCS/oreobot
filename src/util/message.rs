#![allow(unused)]

pub mod emoji {
    pub const CURVED: &str = "1034653422416302151";
    pub const STRAIGHT: &str = "1034653871613681714";
    pub const PIN_EMOJI: &str = "📌";

    pub fn create<S0, S1>(id: S0, name: S1) -> String
    where
        S0: ToString,
        S1: ToString,
    {
        return format!("<:{}:{}>", name.to_string(), id.to_string());
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

    pub async fn register(ctx: &serenity::Context) -> Result<(), MessageCloneError> {
        let to_resync: Vec<prisma::prisma_client::message_pin::Data> = vec![];

        let wh_id = ctx
            .cache
            .guild(nci::ID)
            .make_error(MessageCloneError::NciNotFound)?
            .webhooks(&ctx)
            .await?
            .into_iter()
            .find(|n| {
                let Some(name) = n.name.as_ref() else {
                    return false;
                };

                name == "Oreo's Internals"
            })
            .make_error(MessageCloneError::NoWebhook)?;

        for clone in to_resync {
            let ctx = ctx.clone();

            async_non_blocking!({
                clone_listen(
                    &ctx,
                    clone.pinned_message_id.parse().unwrap(),
                    clone.original_id.parse().unwrap(),
                    wh_id.id.0,
                )
                .await
                .unwrap();
            });
        }

        Ok(())
    }

    #[derive(Clone)]
    pub struct CloneArgs {
        pub jump_btn: bool,
        pub reply: bool,
        pub destination: serenity::ChannelId,
        pub add_rows: Vec<serenity::CreateActionRow>,
        pub sync: bool,
        pub clone_as: Option<serenity::User>,
        pub message: serenity::Message,
        pub ctx: serenity::Context,
        pub ping: bool,
    }

    #[derive(Clone)]
    pub struct CloneArgsBuilder {
        jump_btn: bool,
        reply: bool,
        destination: Option<serenity::ChannelId>,
        add_rows: Vec<serenity::CreateActionRow>,
        sync: bool,
        clone_as: Option<serenity::User>,
        message: Option<serenity::Message>,
        ctx: Option<serenity::Context>,
        ping: bool,
    }

    #[allow(dead_code)]
    impl CloneArgsBuilder {
        pub fn build_from(
            builder: impl Fn(&mut CloneArgsBuilder),
        ) -> Result<CloneArgs, UnfinishedBuilderError> {
            let mut args = CloneArgsBuilder {
                jump_btn: true,
                reply: true,
                destination: None,
                add_rows: vec![],
                sync: false,
                clone_as: None,
                message: None,
                ctx: None,
                ping: false,
            };

            builder(&mut args);
            args.build()
        }

        pub fn jump_btn(&mut self, jump_btn: bool) -> &mut Self {
            self.jump_btn = jump_btn;
            self
        }

        pub fn reply(&mut self, reply: bool) -> &mut Self {
            self.reply = reply;
            self
        }

        pub fn destination(&mut self, destination: serenity::ChannelId) -> &mut Self {
            self.destination = Some(destination);
            self
        }

        pub fn add_rows(&mut self, add_rows: Vec<serenity::CreateActionRow>) -> &mut Self {
            self.add_rows = add_rows;
            self
        }

        pub fn sync(&mut self, sync: bool) -> &mut Self {
            self.sync = sync;
            self
        }

        pub fn clone_as(&mut self, clone_as: serenity::User) -> &mut Self {
            self.clone_as = Some(clone_as);
            self
        }

        pub fn message(&mut self, message: serenity::Message) -> &mut Self {
            self.message = Some(message);
            self
        }

        pub fn ctx(&mut self, ctx: serenity::Context) -> &mut Self {
            self.ctx = Some(ctx);
            self
        }

        pub fn ping(&mut self, ping: bool) -> &mut Self {
            self.ping = ping;
            self
        }

        fn build(self) -> Result<CloneArgs, UnfinishedBuilderError> {
            let destination_provided = self.destination.is_some();
            let message_provided = self.message.is_some();
            let ctx_provided = self.ctx.is_some();

            if !destination_provided || !message_provided {
                let mut missing = vec![];

                if !destination_provided {
                    missing.push("destination");
                }

                if !message_provided {
                    missing.push("message");
                }

                if !ctx_provided {
                    missing.push("ctx");
                }

                bail!(UnfinishedBuilderError(missing));
            }

            let CloneArgsBuilder {
                jump_btn,
                reply,
                destination,
                add_rows,
                sync,
                clone_as,
                message,
                ctx,
                ping,
            } = self;

            let args = CloneArgs {
                jump_btn,
                reply,
                destination: destination.unwrap(),
                add_rows,
                sync,
                clone_as,
                message: message.unwrap(),
                ctx: ctx.unwrap(),
                ping,
            };

            Ok(args)
        }
    }

    pub async fn clone(args: CloneArgs) -> Result<serenity::Message, MessageCloneError> {
        let CloneArgs {
            jump_btn,
            reply,
            destination,
            mut add_rows,
            sync,
            clone_as,
            message,
            ctx,
            ping,
        } = args;

        let mut wh_message = serenity::ExecuteWebhook::default();
        let mut wh_row = serenity::CreateActionRow::default();
        let mut wh_content = "".to_string();
        let clone_as = clone_as.as_ref().unwrap_or(&message.author);

        if let Some(av) = clone_as.avatar_url() {
            wh_message.avatar_url(av);
        }

        let member = ctx
            .cache
            .guild(nci::ID)
            .make_error(MessageCloneError::NciNotFound)?
            .member(&ctx, clone_as.id)
            .await?;

        let username = member.display_name();

        wh_message.username(username);

        let mut row_has_content = false;

        if jump_btn {
            wh_row.create_button(|btn| {
                btn.style(serenity::ButtonStyle::Link);
                btn.label("Jump to original");
                btn.url(message.link());
                btn
            });

            row_has_content = true;
        }

        if reply {
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

                row_has_content = true;
            }
        }

        if !row_has_content {
            if add_rows.len() > 0 {
                wh_row = add_rows.remove(0);
                row_has_content = true;
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

        if !ping {
            wh_message.allowed_mentions(|f| f.empty_users().empty_roles());
        }

        all_rows.push(wh_row);
        all_rows.append(&mut add_rows);
        components.set_action_rows(all_rows);

        if row_has_content {
            wh_message.set_components(components);
        }

        let maybe_webhook = destination.webhooks(&ctx).await?.into_iter().find(|n| {
            let Some(name) = n.name.as_ref() else {
                return false;
            };

            name == "Oreo's Internals"
        });

        let webhook = if let Some(wh) = maybe_webhook {
            wh
        } else {
            destination.create_webhook(&ctx, "Oreo's Internals").await?
        };

        let cloned = webhook
            .execute(&ctx, true, |exec| {
                exec.clone_from(&wh_message);
                exec
            })
            .await?
            .make_error(MessageCloneError::NoWebhookMessage)?;

        if sync {
            let cloned_id = cloned.id.0;
            let wh_id = webhook.id.0;
            let from_id = message.id.0;

            clone_listen(&ctx, cloned_id, from_id, wh_id).await?;
        }

        Ok(cloned)
    }

    async fn clone_listen(
        ctx: &serenity::Context,
        wh_message_id: u64,
        original_id: u64,
        webhook_id: u64,
    ) -> Result<(), MessageCloneError> {
        let data_arc = data::get_serenity(ctx).await;
        let mut data = data_arc.lock().await;
        let emitter = &mut data.emitter;

        emitter.on_filter(
            events::MessageUpdateEvent,
            move |ev_message, ctx| async move {
                let webhook = ctx
                    .cache
                    .guild(nci::ID)
                    .make_error(anyhow!("Could not find NCI"))?
                    .webhooks(&ctx)
                    .await?
                    .into_iter()
                    .find(|wh| wh.id == webhook_id)
                    .make_error(anyhow!("Could not find webhook"))?;

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

        emitter.on_filter(
            events::MessageDeleteEvent,
            move |_, ctx| async move {
                ctx.cache
                    .guild(nci::ID)
                    .make_error(anyhow!("Could not find guild"))?
                    .webhooks(&ctx)
                    .await?
                    .iter()
                    .find(|wh| wh.id.0 == webhook_id)
                    .make_error(anyhow!("Could not find webhook"))?
                    .delete_message(&ctx, serenity::MessageId(wh_message_id))
                    .await?;

                Ok(())
            },
            move |payload| payload.message_id.0 == original_id,
        );

        Ok(())
    }
}
