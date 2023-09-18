use crate::prelude::*;

pub async fn create(role: serenity::Role) -> Result<(), LoggingError> {
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

pub async fn update(role: serenity::Role) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    if nci::roles::can_log(role.id) {
        bail!(LoggingError::Blacklisted(format!("role with id {}", role.id)));
    }

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

pub async fn delete(role: serenity::RoleId, ctx: serenity::Context) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;
    let nci = ctx
        .cache
        .guild(nci::ID)
        .make_error(LoggingError::NciNotFound)?;

    let prisma_role = prisma
        .role()
        .find_unique(role::id::equals(role.to_string()))
        .with(role::users::fetch(
            vec![], /* todo: check if this works and if not, replace */
        ))
        .exec()
        .await?
        .unwrap();

    if let Some(mut user) = 'blk: {
        if !prisma_role.color_role {
            break 'blk None;
        };

        let Some(user_data) = prisma_role.users()?.first() else {
            break 'blk None;
        };

        if user_data.removed {
            break 'blk None;
        };

        let Ok(user) = nci.member(&ctx, user_data.id.parse::<u64>().unwrap()).await else {
            break 'blk None;
        };

        Some(user)
    } {
        let color = Color::from_hex(prisma_role.color)?.into();
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
                vec![role::deleted::set(true), role::users::set(vec![])],
            )
            .exec()
            .await?;
    }

    Ok(())
}
