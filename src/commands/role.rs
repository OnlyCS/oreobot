use crate::prelude::*;

#[poise::command(slash_command)]
pub async fn role(
    ctx: Context<'_>,
    #[description = "Change the role name"] name: Option<String>,
    #[description = "Change the role color"] color: Option<Color>,
    #[description = "For this user (admin)"] user: Option<serenity::Member>,
) -> Result<(), CommandError> {
    let mut loading = Loading::<LoadingWithInteraction>::new(
        &ctx,
        "Locking cache. Depending on how long the bot has been up, this may take a while.",
    )
    .await?;

    let prisma = prisma::create().await?;
    let data_arc = data::get_poise(&ctx);
    let mut data = data_arc.lock().await;
    let cache = &mut data.cache;

    loading.update(&ctx, "Updating your role settings").await?;

    let user = if let Some(member) = user.as_ref() {
        if is_admin::member(&prisma, &member).await? {
            &member.user
        } else {
            loading.close(&ctx).await?;

            bail!(CommandError::RuntimeError {
                title: "Admin",
                description: "You must be an admin to change other users' roles"
            });
        }
    } else {
        ctx.author()
    };

    let mut confirmation = embed::default(&ctx, EmbedStatus::Sucess);
    confirmation.title("Role > Set");
    confirmation.description("Sucessfully updated your role");

    if let Some(name) = name {
        confirmation.field("Name", &name, true);

        cache
            .update::<cache_items::RoleName>(ctx.serenity_context().clone(), (user.id, name))
            .await?;
    }

    if let Some(color) = color {
        cache
            .update::<cache_items::RoleColor>(ctx.serenity_context().clone(), (user.id, color))
            .await?;

        confirmation.field("Color", color.into_hex(), true);
    }

    loading.last(&ctx, confirmation).await?;

    Ok(())
}
