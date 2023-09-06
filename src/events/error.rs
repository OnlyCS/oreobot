use crate::prelude::*;

pub async fn handle(
    framework_error: poise::FrameworkError<'_, Shared<Data>, CommandError>,
) -> Result<(), serenity::Error> {
    match &framework_error {
        poise::FrameworkError::Command { error, ctx } => {
            let mut embed = embed::default(
                &ctx,
                if let CommandError::RuntimeWarning { .. } = error {
                    embed::EmbedStatus::Warning
                } else {
                    embed::EmbedStatus::Error
                },
            );

            match error {
                CommandError::RuntimeWarning { title, description } => {
                    embed.title(format!("{} > Warning > {}", ctx.command().name, title));
                    embed.description(format!("Warning in command {}\n{}\nDon't worry, this warning has been appropriately handled and the bot will continue to function normally", ctx.command().name, description));
                }
                CommandError::RuntimeError { title, description } => {
                    embed.title(format!("{} > Error > {}", ctx.command().name, title));
                    embed.description(format!("Warning in command {}\n{}\nDon't worry, this error has been appropriately handled and the bot will continue to function normally", ctx.command().name, description));
                }
                _ => {
                    embed.title(format!("{} > Error", ctx.command().name));
                    embed.description(format!("{}\nDon't worry, this error has been appropriately handled and the bot will continue to function normally", error.to_string()));
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
