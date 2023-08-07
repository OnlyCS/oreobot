pub mod emoji {
    pub const CURVED: &str = "1034653422416302151";
    pub const STRAIGHT: &str = "1034653871613681714";

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

    pub struct CloneLinkData {
        link: bool,
        reply: bool,
        link_text: &'static str,
    }

    impl Default for CloneLinkData {
        fn default() -> Self {
            Self {
                link: true,
                reply: true,
                link_text: "Jump to original",
            }
        }
    }

    pub async fn clone(
        ctx: &Context<'_>,
        message: serenity::Message,
        destination: serenity::GuildChannel,
        options: CloneLinkData,
    ) -> Result<()> {
        let emitter_mutex = Arc::clone(&ctx.data().emitter);
        let mut emitter = emitter_mutex.lock().await;

        let webhooks = destination.webhooks(&ctx).await?;
        let webhook = match webhooks.first() {
            Some(n) => n.clone(),
            None => destination.create_webhook(&ctx, "OreoBot").await?,
        };

        let attachments = (&message.attachments)
            .into_iter()
            .map(|n| &n.url)
            .collect::<Vec<_>>();

        let author = &message.author;
        let content = &message.content;
        let partial_member = message.member.as_ref().context("Member not fetched")?;
        let url = &message.link();

        let member = ctx
            .serenity_context()
            .cache
            .member(
                partial_member
                    .guild_id
                    .as_ref()
                    .context("Could not get member data")?,
                author.id,
            )
            .context("Could not get member data")?;

        let reference = message.referenced_message;
        let kind = message.kind;

        let reply_text = if options.reply {
            if let Some(reference) = reference && kind == serenity::MessageType::InlineReply {
				let author = reference.author;
				let mut content = reference.content;
				content.truncate(43);
				content.push_str("...");

				let mention = mention::create(author.id, mention::MentionType::User);

				format!("{} {}\t{}\n{}\n", emoji::create(emoji::CURVED, "curved"), mention, content, emoji::create(emoji::STRAIGHT, "straight"))
			} else {
				"".to_string()
			}
        } else {
            "".to_string()
        };

        let wh_message = webhook
            .execute(&ctx, false, |executer| {
                executer.content(format!("{}{}", reply_text, content));

                if let Some(avatar) = member.avatar_url() {
                    executer.avatar_url(avatar);
                }

                for attachment in attachments {
                    executer.add_file(attachment as &str);
                }

                executer.username(member.display_name());

                if options.link {
                    executer.components(|component_creator| {
                        component_creator.create_action_row(|ar_creator| {
                            ar_creator.create_button(|btn_creator| {
                                btn_creator
                                    .label(options.link_text)
                                    .url(url)
                                    .style(serenity::ButtonStyle::Link)
                            })
                        })
                    });
                }

                executer
            })
            .await?;

        // todo: database this so we can always re-add event listeners
        if let Some(wh_message) = wh_message {
            let wh_message_id = wh_message.id.0; // u64 has copy!
            let wh_id = webhook.id.0;
            let wh_gid = webhook.guild_id.unwrap().0;

            emitter.on_async_filter(
                event::MessageUpdateEvent,
                move |message, ctx: serenity::Context| async move {
                    ctx.cache
                        .guild(wh_gid)
                        .context("Could not find guild")?
                        .webhooks(&ctx)
                        .await?
                        .iter()
                        .find(|n| n.id.0 == wh_id)
                        .context("Could not get webhook to edit message")?
                        .edit_message(&ctx, serenity::MessageId(wh_message_id), |msg| {
                            if let Some(content) = message.content {
                                msg.content(content);
                            }

                            msg
                        })
                        .await?;

                    Ok(())
                },
                move |message| message.id == wh_message_id,
            );

            emitter.on_async_filter(
                event::MessageDeleteEvent,
                move |_, ctx: serenity::Context| async move {
                    ctx.cache
                        .guild(wh_gid)
                        .context("Could not find guild")?
                        .webhooks(&ctx)
                        .await?
                        .iter()
                        .find(|n| n.id.0 == wh_id)
                        .context("Could not get webhook to delete message")?
                        .delete_message(&ctx, serenity::MessageId(wh_message_id))
                        .await?;

                    Ok(())
                },
                move |payload| payload.message_id == wh_message_id,
            )
        }

        Ok(())
    }
}
