use crate::prelude::*;

pub async fn handle(framework_error: poise::FrameworkError<'_, Data, anyhow::Error>) -> Result<()> {
    match &framework_error {
        poise::FrameworkError::Command { error, ctx } => {
            ctx.send(|reply| {
                let mut embed = embed::default(&ctx, embed::EmbedStatus::Error);

				embed.title("Error".to_string());

                embed.description(format!(
                    "{}: {}\nDon't worry, this error has been appropriately handled and the bot will continue to function normally",
                    framework_error.to_string().capitalize_first_letter(),
                    error.to_string()
                ));

                reply.embed(|creator| {
                    creator.clone_from(&embed);

                    creator
                });

                reply.components(|comp| comp.add_action_row(share::row(false)));
                reply.ephemeral(true);

                reply
            })
            .await?;
        }
        _ => {}
    }

    Ok(())
}
