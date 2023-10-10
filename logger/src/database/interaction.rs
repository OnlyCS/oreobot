use crate::prelude::*;

async fn command(interaction: serenity::CommandInteraction) -> Result<(), InteractionLogError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id,
            InteractionType::Command,
            interaction.token.to_string(),
            interaction.application_id,
            channel::id::equals(interaction.channel_id),
            user::id::equals(interaction.user.id),
            vec![],
        )
        .exec()
        .await?;

    prisma
        .command_interaction_data()
        .create(
            interaction.data.id.to_string(),
            interaction.data.name.to_string(),
            interaction::id::equals(interaction.id),
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

async fn message_component(
    interaction: serenity::ComponentInteraction,
) -> Result<(), InteractionLogError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id,
            InteractionType::MessageComponent,
            &interaction.token,
            interaction.application_id,
            channel::id::equals(interaction.channel_id),
            user::id::equals(interaction.user.id),
            vec![interaction::custom_id::set(Some(
                interaction.data.custom_id.clone(),
            ))],
        )
        .exec()
        .await?;

    Ok(())
}

async fn modal_submit(interaction: serenity::ModalInteraction) -> Result<(), InteractionLogError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id,
            InteractionType::ModalSubmit,
            interaction.token.to_string(),
            interaction.application_id,
            channel::id::equals(interaction.channel_id),
            user::id::equals(interaction.user.id),
            vec![interaction::custom_id::set(Some(
                interaction.data.custom_id.clone(),
            ))],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn create(interaction: serenity::Interaction) -> Result<(), InteractionLogError> {
    match interaction.kind() {
        serenity::InteractionType::Command => {
            command(interaction.command().unwrap()).await?;
        }
        serenity::InteractionType::Component => {
            message_component(interaction.message_component().unwrap()).await?;
        }
        serenity::InteractionType::Modal => {
            modal_submit(interaction.modal_submit().unwrap()).await?;
        }
        _ => {}
    };

    Ok(())
}
