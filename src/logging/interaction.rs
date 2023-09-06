use crate::prelude::*;

pub async fn command(
    interaction: serenity::ApplicationCommandInteraction,
) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id.to_string(),
            InteractionType::Command,
            channel::id::equals(interaction.channel_id.to_string()),
            user::id::equals(interaction.user.id.to_string()),
            vec![],
        )
        .exec()
        .await?;

    prisma
        .command_interaction_data()
        .create(
            interaction.data.id.to_string(),
            interaction.data.name,
            interaction::id::equals(interaction.id.to_string()),
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn message_component(
    interaction: serenity::MessageComponentInteraction,
) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id.to_string(),
            InteractionType::MessageComponent,
            channel::id::equals(interaction.channel_id.to_string()),
            user::id::equals(interaction.user.id.to_string()),
            vec![interaction::custom_id::set(Some(
                interaction.data.custom_id,
            ))],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn modal_submit(
    interaction: serenity::ModalSubmitInteraction,
) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id.to_string(),
            InteractionType::ModalSubmit,
            channel::id::equals(interaction.channel_id.to_string()),
            user::id::equals(interaction.user.id.to_string()),
            vec![interaction::custom_id::set(Some(
                interaction.data.custom_id,
            ))],
        )
        .exec()
        .await?;

    Ok(())
}
