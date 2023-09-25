use crate::prelude::*;

const CACHE_MSG_NOT_FOUND: &'static str = "
This message does not seem to be in the cache. 
This could be because:

- Smarty is down
- My internet is slow
- This message was created before I started logging news-in-chat messages.
 
Please wait a few seconds and try again.";

#[poise::command(context_menu_command = "News to Chat")]
pub async fn jump_to_chat(
    ctx: Context<'_>,
    message: serenity::Message,
) -> Result<(), CommandError> {
    if message.channel_id != nci::channels::NEWS {
        bail!(CommandError::RuntimeError {
            title: "Wrong channel",
            description: "This context menu command can only be used in the news channel"
        });
    }

    let data_arc = data::get_poise(&ctx);
    let mut data = data_arc.lock().await;
    let cache = &mut data.cache;

    let chat_msg_id = cache
        .get::<cache_items::NewsInChat>(ctx.serenity_context().clone(), message.clone())
        .await?;

    let Some(chat_msg_id) = chat_msg_id else {
        bail!(CommandError::RuntimeWarning {
            title: "Message not in cache",
            description: CACHE_MSG_NOT_FOUND
        });
    };

    let chat_message_link = format!(
        "https://discordapp.com/channels/{}/{}/{}",
        nci::ID,
        nci::channels::CHAT,
        chat_msg_id
    );

    let mut embed = embed::default(&ctx, EmbedStatus::Success);
    embed.title("Jump to Chat > Link");
    embed.description(
        "Sorry, you have to click two buttons :(\n Discord should add context menu links.",
    );

    let mut components = serenity::CreateComponents::default();
    let mut row = serenity::CreateActionRow::default();
    let mut button = serenity::CreateButton::default();

    button.style(serenity::ButtonStyle::Link);
    button.label("Jump to Chat");
    button.url(chat_message_link);

    row.add_button(button);
    components.add_action_row(row);
    components.add_action_row(share::row(false));

    ctx.send(|creator| {
        creator.embeds.push(embed);
        creator.components = Some(components);
        creator.ephemeral(true);
        creator
    })
    .await?;

    Ok(())
}
