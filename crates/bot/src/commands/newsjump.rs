use crate::prelude::*;

const ENOTFOUND: &str = "
This message was not found in the database.
This is most likely due to bot downtime.
Do not contact a developer for more information.
";

#[poise::command(context_menu_command = "Jump to #chat")]
pub async fn newsjump(ctx: Context<'_>, message: Message) -> Result<(), CommandError> {
    if message.channel_id != nci::channels::NEWS {
        bail!(CommandError::IllegalArgument(String::from(
            "This command can only be used in #news"
        )));
    }

    let mut logger = Client::<LoggingServer>::new().await?;

    let LoggingResponse::MessageClonesOk(clones) = logger
        .send(LoggingRequest::MessageCloneRead { source: message.id })
        .await?
    else {
        bail!(RouterError::<LoggingServer>::InvalidResponse);
    };

    if let Some(clone) = clones
        .into_values()
        .filter(|clone| clone.reason == MessageCloneReason::NewsInChat)
        .next()
    {
        let message_id = MessageId::new(clone.id as u64);
        let link = message_id.link(nci::channels::CHAT, message.guild_id);
        let button = CreateButton::new_link(link).label("Jump");
        let row = CreateActionRow::Buttons(vec![button, share::button()]);
        let reply = poise::CreateReply::default()
            .components(vec![row])
            .ephemeral(true);

        ctx.send(reply).await?;
    } else {
        bail!(CommandError::IllegalArgument(String::from(ENOTFOUND)))
    }

    Ok(())
}
