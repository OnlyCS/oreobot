pub use crate::prelude::*;

pub async fn join(mut member: serenity::Member, ctx: serenity::Context) -> Result<()> {
    get_prisma::from_serenity_context!(prisma, ctx);

    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;

    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(member.user.id.to_string()))
        .with(user::color_role::fetch())
        .exec()
        .await?;

    let color_role = if member.user.bot {
        member
            .roles(&ctx)
            .unwrap_or(vec![])
            .iter()
            .find(|role| role.managed)
            .context("Bot doesnt have managed role wtf?")?
            .clone()
    } else {
        if let Some(prisma_user) = prisma_user.as_ref() {
            let color_role = prisma_user.color_role()?;
            let color = colors::hex_to_color(color_role.color.clone())?;

            let role = nci
                .create_role(&ctx, |r| {
                    r.name(color_role.name.clone())
                        .colour(color.0.into())
                        .mentionable(false)
                })
                .await?;

            member.add_role(&ctx, role.id).await?;

            prisma
                .role()
                .update(
                    role::id::equals(color_role.id.to_string()),
                    vec![
                        role::id::set(role.id.to_string()),
                        role::deleted::set(false),
                        role::users::connect(vec![user::id::equals(member.user.id.to_string())]),
                    ],
                )
                .exec()
                .await?;

            role
        } else {
            let role = nci
                .create_role(&ctx, |r| {
                    r.clone_from(&default_role(&member).unwrap());
                    r
                })
                .await?;

            member.add_role(&ctx, role.id).await?;

            prisma
                .role()
                .create(
                    role.id.to_string(),
                    role.name.clone(),
                    role.colour.hex(),
                    vec![role::color_role::set(true)],
                )
                .exec()
                .await?;

            role
        }
    };

    if let Some(prisma_user) = prisma_user {
        let mut updates = vec![];

        updates.push(user::color_role::connect(role::id::equals(
            color_role.id.to_string(),
        )));

        updates.push(user::roles::connect(vec![role::id::equals(
            color_role.id.to_string(),
        )]));

        updates.push(user::removed::set(false));

        if prisma_user.username != member.user.name {
            updates.push(user::username::set(member.user.name.clone()));
        }

        if prisma_user.nickname != member.nick {
            updates.push(user::nickname::set(member.nick.clone()));
        }

        prisma
            .user()
            .update(user::id::equals(prisma_user.id), updates)
            .exec()
            .await?;

        let mut new_roles = vec![];

        if prisma_user.admin {
            new_roles.push(nci::roles::OVERRIDES);
        }

        if prisma_user.verified {
            new_roles.push(nci::roles::MEMBERS)
        }

        if prisma_user.bot {
            new_roles.push(nci::roles::BOTS);
        }

        if !new_roles.is_empty() {
            member.add_roles(&ctx, &new_roles).await?;
        }
    } else {
        prisma
            .user()
            .create(
                member.user.id.to_string(),
                member.user.name.clone(),
                role::id::equals(color_role.id.to_string()),
                vec![user::roles::connect(vec![role::id::equals(
                    color_role.id.to_string(),
                )])],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn update(member: serenity::Member, ctx: serenity::Context) -> Result<()> {
    get_prisma::from_serenity_context!(prisma, ctx);

    let mut updates = vec![];

    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;
    let member_roles = member.roles(&ctx).unwrap_or(vec![]);
    let prisma_member = prisma
        .user()
        .find_unique(user::id::equals(member.user.id.to_string()))
        .exec()
        .await?
        .context("Could not find user in database")?;

    let mut role_connects = member_roles
        .iter()
        .map(|n| role::id::equals(n.id.to_string()))
        .collect::<Vec<_>>();

    if !member_roles
        .iter()
        .any(|n| n.id.to_string() == prisma_member.color_role_id)
    {
        let color_role = prisma_member.color_role_id;

        nci.member(&ctx, member.user.id)
            .await?
            .add_role(&ctx, color_role.parse::<u64>()?)
            .await?;

        role_connects.push(role::id::equals(color_role));
    }

    if member.user.name != prisma_member.username {
        updates.push(user::username::set(member.user.name.clone()));
    }

    if member.nick != prisma_member.nickname {
        updates.push(user::nickname::set(member.nick.clone()));
    }

    updates.push(user::roles::connect(role_connects));

    prisma
        .user()
        .update(user::id::equals(prisma_member.id), updates)
        .exec()
        .await?;

    Ok(())
}

pub async fn leave(id: serenity::UserId, ctx: serenity::Context) -> Result<()> {
    get_prisma::from_serenity_context!(prisma, ctx);

    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(id.to_string()))
        .exec()
        .await?
        .context("Could not find user in database")?;

    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;

    // this will send a roledelete event, handled in logging/role.rs
    nci.delete_role(&ctx, prisma_user.color_role_id.parse::<u64>()?)
        .await?;

    prisma
        .user()
        .update(
            user::id::equals(id.to_string()),
            vec![user::removed::set(true), user::roles::set(vec![])], /* color role relation is still preserved in case the user re-joins */
        )
        .exec()
        .await?;

    Ok(())
}
