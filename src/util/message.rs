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

        let button_cid = format!("message_link:{}", uuid::Uuid::new_v4());

        webhook
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
                                    .custom_id(button_cid)
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

        Ok(())
    }
}
