use crate::prelude::*;

pub async fn update(role: serenity::Role, ctx: serenity::Context) -> Result<()> {
    let data = &ctx.data;
    let prisma_mutex = Arc::clone(
        data.read()
            .await
            .get::<PrismaTypeKey>()
            .context("Could not find prismaclient in data")?,
    );

    let prisma = prisma_mutex.lock().await;

    if let Some(role_data) = prisma
        .user_role()
        .find_unique(user_role::id::equals(role.id.to_string()))
        .exec()
        .await?
    {
        let mut updates = vec![];

        if role.colour.hex() != role_data.color {
            updates.push(user_role::color::set(role.colour.hex()));
        }

        if role.name != role_data.name {
            updates.push(user_role::name::set(role.name));
        }

        if !updates.is_empty() {
            prisma
                .user_role()
                .update(user_role::id::equals(role.id.to_string()), updates)
                .exec()
                .await?;
        }
    }

    Ok(())
}

pub async fn delete(id: serenity::RoleId, ctx: serenity::Context) -> Result<()> {
    let data = &ctx.data;
    let prisma_mutex = Arc::clone(
        data.read()
            .await
            .get::<PrismaTypeKey>()
            .context("Could not find prismaclient in data")?,
    );
    let prisma = prisma_mutex.lock().await;

    if let Some(role_data) = prisma
        .user_role()
        .find_unique(user_role::id::equals(id.to_string()))
        .with(user_role::user::fetch())
        .exec()
        .await?
    {
        // oh shit something went wrong

        let user_data = role_data.user()?.context("No user found somehow")?;
        let user_id = &user_data.id;

        let guild = ctx
            .cache
            .guild(nci::ID.parse::<u64>()?)
            .context("Could not find nci")?;

        let role = guild
            .create_role(&ctx, |role| {
                role.name(user_data.nickname.as_ref().unwrap_or(&user_data.username))
                    .colour(serenity::Colour::RED.0.into())
                    .position(5)
                    .mentionable(false)
            })
            .await?;

        let mut user = guild.member(&ctx, user_id.parse::<u64>()?).await?;

        user.add_role(&ctx, role.id).await?;

        prisma
            .user_role()
            .update(
                user_role::id::equals(role_data.id.to_string()),
                vec![
                    user_role::id::set(role.id.to_string()),
                    user_role::name::set(role.name),
                    user_role::color::set(role.colour.hex()),
                ],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn set_admin(
    user: serenity::UserId,
    is_admin: bool,
    ctx: serenity::Context,
) -> Result<()> {
    let data = &ctx.data;
    let prisma_mutex = Arc::clone(
        data.read()
            .await
            .get::<PrismaTypeKey>()
            .context("Could not find prismaclient in data")?,
    );
    let prisma = prisma_mutex.lock().await;

    prisma
        .user()
        .update(
            user::id::equals(user.to_string()),
            vec![user::admin::set(is_admin)],
        )
        .exec()
        .await?;

    Ok(())
}
