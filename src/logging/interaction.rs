use crate::prelude::*;

// prepare for sphagetti code mess

pub async fn create(interaction: serenity::Interaction, ctx: serenity::Context) -> Result<()> {
    let data = &ctx.data;
    let prisma_mutex = Arc::clone(
        data.read()
            .await
            .get::<PrismaTypeKey>()
            .context("Could not find prismaclient in data")?,
    );
    let prisma = prisma_mutex.lock().await;

    let id = interaction.id().to_string();
    let channel_id;
    let invoker_id;
    let custom_id;

    let kind = match interaction.kind() {
        serenity::InteractionType::ApplicationCommand => {
            let interaction = interaction.as_application_command().unwrap();

            channel_id = interaction.channel_id;
            invoker_id = match interaction.member.as_ref() {
                Some(n) => n.user.id,
                None => return Ok(()),
            };

            if interaction.user.bot {
                return Ok(());
            }

            custom_id = None;

            InteractionType::Command
        }
        serenity::InteractionType::ModalSubmit => {
            let interaction = interaction.as_modal_submit().unwrap();

            channel_id = interaction.channel_id;
            invoker_id = match interaction.member.as_ref() {
                Some(n) => n.user.id,
                None => return Ok(()),
            };

            if interaction.user.bot {
                return Ok(());
            }

            custom_id = Some(interaction.data.custom_id.clone());

            InteractionType::ModalSubmit
        }
        serenity::InteractionType::MessageComponent => {
            let interaction = interaction.as_message_component().unwrap();

            channel_id = interaction.channel_id;
            invoker_id = match interaction.member.as_ref() {
                Some(n) => n.user.id,
                None => return Ok(()),
            };

            if interaction.user.bot {
                return Ok(());
            }

            custom_id = Some(interaction.data.custom_id.clone());

            match interaction.data.component_type {
                serenity::ComponentType::Button => InteractionType::Button,
                serenity::ComponentType::InputText => InteractionType::TextInput,
                serenity::ComponentType::SelectMenu => InteractionType::Select,
                _ => return Ok(()),
            }
        }
        _ => return Ok(()),
    };

    prisma
        .interaction()
        .create(
            id.clone(),
            kind,
            channel::id::equals(channel_id.to_string()),
            user::id::equals(invoker_id.to_string()),
            vec![interaction::custom_id::set(custom_id)],
        )
        .exec()
        .await?;

    if kind == InteractionType::Command {
        // parse extra data

        let interaction = interaction.application_command().unwrap();
        let data = interaction.data;

        let command_id = data.id.to_string();
        let command_name = data.name.to_string();

        prisma
            .command_interaction_data()
            .create(
                command_id,
                command_name,
                interaction::id::equals(id),
                vec![],
            )
            .exec()
            .await?;
    }

    Ok(())
}
