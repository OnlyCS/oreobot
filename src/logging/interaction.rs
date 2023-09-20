use crate::prelude::*;

pub async fn command(
    ctx: serenity::Context,
    interaction: serenity::ApplicationCommandInteraction,
) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id.to_string(),
            InteractionType::Command,
            interaction.token.to_string(),
            interaction.application_id.to_string(),
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
            interaction.data.name.to_string(),
            interaction::id::equals(interaction.id.to_string()),
            vec![],
        )
        .exec()
        .await?;

    // three seconds to respond to an interaction
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    if interaction.get_interaction_response(&ctx).await.is_ok()
        && interaction.application_id == ctx.http.get_current_application_info().await?.id
    {
        prisma
            .interaction()
            .update(
                interaction::id::equals(interaction.id.to_string()),
                vec![interaction::reusable::set(true)],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn message_component(
    ctx: serenity::Context,
    interaction: serenity::MessageComponentInteraction,
) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id.to_string(),
            InteractionType::MessageComponent,
            interaction.token.to_string(),
            interaction.application_id.to_string(),
            channel::id::equals(interaction.channel_id.to_string()),
            user::id::equals(interaction.user.id.to_string()),
            vec![interaction::custom_id::set(Some(
                interaction.data.custom_id.clone(),
            ))],
        )
        .exec()
        .await?;

    // three seconds to respond to an interaction
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    if interaction.get_interaction_response(&ctx).await.is_ok()
        && interaction.application_id == ctx.http.get_current_application_info().await?.id
    {
        prisma
            .interaction()
            .update(
                interaction::id::equals(interaction.id.to_string()),
                vec![interaction::reusable::set(true)],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn modal_submit(
    ctx: serenity::Context,
    interaction: serenity::ModalSubmitInteraction,
) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .interaction()
        .create(
            interaction.id.to_string(),
            InteractionType::ModalSubmit,
            interaction.token.to_string(),
            interaction.application_id.to_string(),
            channel::id::equals(interaction.channel_id.to_string()),
            user::id::equals(interaction.user.id.to_string()),
            vec![interaction::custom_id::set(Some(
                interaction.data.custom_id.clone(),
            ))],
        )
        .exec()
        .await?;

    // three seconds to respond to an interaction
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    if interaction.get_interaction_response(&ctx).await.is_ok()
        && interaction.application_id == ctx.http.get_current_application_info().await?.id
    {
        prisma
            .interaction()
            .update(
                interaction::id::equals(interaction.id.to_string()),
                vec![interaction::reusable::set(true)],
            )
            .exec()
            .await?;
    }

    Ok(())
}
