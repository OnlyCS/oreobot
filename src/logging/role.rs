use crate::prelude::*;

pub async fn create(role: serenity::Role, ctx: serenity::Context) -> Result<()> {
    let prisma = prisma::create().await?;

    prisma
        .role()
        .create(
            role.id.to_string(),
            role.name.clone(),
            role.colour.hex(),
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn update(role: serenity::Role, ctx: serenity::Context) -> Result<()> {
    let prisma = prisma::create().await?;

    prisma
        .role()
        .update(
            role::id::equals(role.id.to_string()),
            vec![
                role::name::set(role.name),
                role::color::set(role.colour.hex()),
            ],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(role: serenity::RoleId, ctx: serenity::Context) -> Result<()> {
    let prisma = prisma::create().await?;

    let prisma_role = prisma
        .role()
        .find_unique(role::id::equals(role.to_string()))
        .with(role::users::fetch(vec![] /* todo: check if this works and if not, replace */))
        .exec()
        .await?
        .unwrap();

    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;

    if prisma_role.color_role 
		&& let Some(color_role_user_data) = prisma_role.users()?.first() 
		&& let Ok(mut user) = nci.member(&ctx,color_role_user_data.id.parse::<u64>()?).await {
        let color = colors::hex_to_color(prisma_role.color.clone())?.0.into();

        let color_role = nci
            .create_role(&ctx, |role| {
                role.name(prisma_role.name.clone())
                    .colour(color)
                    .mentionable(false)
            })
            .await?;

		user.add_role(&ctx, color_role.id).await?;

        prisma
            .role()
            .update(
                role::id::equals(role.to_string()),
                vec![
                    role::id::set(color_role.id.to_string()),
                    role::color_role::set(true),
                ],
            )
            .exec()
            .await?;
    } else {
        prisma
            .role()
            .update(
                role::id::equals(role.to_string()),
                vec![role::deleted::set(true), role::users::set(vec![]) /* if color role, color_role_user relation still exists */],
            )
            .exec()
            .await?;
    }

    Ok(())
}
