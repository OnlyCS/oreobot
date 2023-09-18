use crate::prelude::*;

pub async fn handle(
    framework_error: poise::FrameworkError<'_, Shared<Data>, CommandError>,
) -> Result<(), serenity::Error> {
    match &framework_error {
        poise::FrameworkError::Command { error, ctx } => {
            let mut embed = embed::default(
                &ctx,
                if matches!(error, CommandError::RuntimeWarning { .. }) {
                    EmbedStatus::Warning
                } else {
                    EmbedStatus::Error
                },
            );

            match error {
                CommandError::RuntimeWarning { title, description } => {
                    embed.title(format!("{} > Warning > {}", ctx.command().name, title));
                    embed.description(format!("Warning in command `{}{}`:\n{}\n\nDon't worry, this warning has been appropriately handled and the bot will continue to function normally", if ctx.command().slash_action.is_some() { "/" } else {""}, ctx.command().name, description));
                }
                CommandError::RuntimeError { title, description } => {
                    embed.title(format!(
                        "{} > Error > {}",
                        ctx.command().name.display_case(),
                        title
                    ));
                    embed.description(format!("Error in command `/{}`:\n{}\n\nDon't worry, this error has been appropriately handled and the bot will continue to function normally", ctx.command().name, description));
                }
                _ => {
                    embed.title(format!("{} > Error", ctx.command().name.display_case()));
                    embed.description(format!("Error in command `/{}`:\n{}\n\nDon't worry, this error has been appropriately handled and the bot will continue to function normally", ctx.command().name, error.to_string()));
                }
            }

            ctx.send(|reply| {
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
