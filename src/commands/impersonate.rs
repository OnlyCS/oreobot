use crate::prelude::*;

#[poise::command(slash_command)]
pub async fn impersonate(
    ctx: Context<'_>,
    #[description = "Use yourself to stop"] to_impersonate: serenity::Member,
) -> Result<(), CommandError> {
    if to_impersonate.user.bot {
        bail!(CommandError::RuntimeError {
            title: "Bots",
            description: "Cannot impersonate bots. This has been disabled for the sake of nMarkov"
        });
    }

    let loading = Loading::<LoadingWithInteraction>::new(
        &ctx,
        "Locking cache. Depending on how long the bot has been up, this may take a while.",
    )
    .await?;

    let mut data = ctx.data().lock().await;
    let cache = &mut data.cache;

    if to_impersonate.user.id == ctx.author().id {
        cache
            .update::<cache_items::Impersonation>(
                ctx.serenity_context().clone(),
                (ctx.author().id, None),
            )
            .await?;

        let mut embed = embed::default(&ctx, EmbedStatus::Sucess);

        embed.title(format!("Impersonate > Set > Stop"));
        embed.description("Sucessfully stopped your impersonation");

        loading.last(&ctx, embed).await?;
    } else {
        cache
            .update::<cache_items::Impersonation>(
                ctx.serenity_context().clone(),
                (ctx.author().id, Some(to_impersonate.user.id)),
            )
            .await?;

        let mut embed = embed::default(&ctx, EmbedStatus::Sucess);

        embed.title(format!("Impersonate > Set > {}", to_impersonate.user.tag()));
        embed.description(format!(
            "Sucessfully set your impersonation to `{}`",
            to_impersonate.user.tag()
        ));

        loading.last(&ctx, embed).await?;
    }

    Ok(())
}
