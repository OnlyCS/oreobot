use crate::prelude::*;

const CUSTOM_ID: &'static str = "oreo2_share";

pub fn button() -> CreateButton {
    CreateButton::new(CUSTOM_ID)
        .label("Share")
        .style(ButtonStyle::Secondary)
}

fn _button_disabled() -> CreateButton {
    CreateButton::new(CUSTOM_ID)
        .label("Share")
        .style(ButtonStyle::Secondary)
        .disabled(true)
}

pub fn row() -> CreateActionRow {
    CreateActionRow::Buttons(vec![button()])
}

fn _row_disabled() -> CreateActionRow {
    CreateActionRow::Buttons(vec![_button_disabled()])
}

pub async fn register() {
    mpmc::on(|ctx, event, _data| async move {
        let FullEvent::InteractionCreate {
            interaction: serenity::Interaction::Component(mut press),
        } = event
        else {
            bail!(EventError::UnwantedEvent)
        };

        if press.data.custom_id != CUSTOM_ID {
            bail!(EventError::UnwantedEvent)
        }

        press
            .create_response(
                &ctx,
                serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::default().add_embeds(
                        press
                            .message
                            .embeds
                            .iter()
                            .cloned()
                            .map(CreateEmbed::from)
                            .collect_vec(),
                    ),
                ),
            )
            .await?;

        press
            .message
            .edit(
                &ctx,
                serenity::EditMessage::default().components(vec![_row_disabled()]),
            )
            .await?;

        Ok(())
    })
    .await
}
