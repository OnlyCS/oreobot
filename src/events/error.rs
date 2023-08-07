use crate::prelude::*;

pub async fn handle(framework_error: poise::FrameworkError<'_, Data, anyhow::Error>) -> Result<()> {
    match &framework_error {
        poise::FrameworkError::Command { error, ctx } => {
            if error.to_string().starts_with("Warning: ") {
				ctx.send(|reply| {
					let mut embed = embed::default(&ctx, embed::EmbedStatus::Warning);
	
					embed.title("Warning".to_string());

					let mut error_desc = framework_error.to_string().capitalize_first_letter().split_at(5).1.to_string();
					error_desc.insert_str(0, "Warning");
	
					embed.description(format!(
						"{}: {}\nDon't worry, this warning has been appropriately handled and the bot will continue to function normally",
						error_desc,
						error.to_string().split_at(9).1
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
			} else {
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
        }
        _ => {}
    }

    Ok(())
}
