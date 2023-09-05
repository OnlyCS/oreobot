pub use crate::prelude::*;

pub async fn join(mut member: serenity::Member, ctx: serenity::Context) -> Result<()> {
    let prisma = prisma::create().await?;
    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;

    let user_if_exists = prisma
        .user()
        .find_unique(user::id::equals(member.user.id.to_string()))
        .exec()
        .await?;

    // take care of roles and color roles
    let color_role = nci
        .create_role(&ctx, |create| create.name(member.display_name()))
        .await?;

    let mut roles_to_add = vec![color_role.id];
    if let Some(user) = &user_if_exists {
        if user.admin {
            roles_to_add.push(nci::roles::OVERRIDES);
        }

        if user.verified {
            roles_to_add.push(nci::roles::MEMBERS);
        }

        if user.bot {
            roles_to_add.push(nci::roles::BOTS);
        }
    }
    member.add_roles(&ctx, &roles_to_add).await?;

    if let Some(user) = &user_if_exists {
        prisma
            .user()
            .update(
                user::id::equals(user.id.clone()),
                vec![
                    user::username::set(member.user.name),
                    user::nickname::set(member.nick),
                    user::roles::set(
                        roles_to_add
                            .into_iter()
                            .map(|n| role::id::equals(n.to_string()))
                            .collect_vec(),
                    ),
                    user::removed::set(false),
                ],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn update(member: serenity::Member, ctx: serenity::Context) -> Result<()> {
    let prisma = prisma::create().await?;

    let mut updates = vec![];

    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;
    let member_roles = member.roles(&ctx).unwrap_or(vec![]);
    let prisma_member = prisma
        .user()
        .find_unique(user::id::equals(member.user.id.to_string()))
        .with(user::roles::fetch(vec![role::color_role::equals(true)]))
        .exec()
        .await?
        .context("Could not find user in database")?;

    let mut role_connects = member_roles
        .iter()
        .map(|n| role::id::equals(n.id.to_string()))
        .collect::<Vec<_>>();

    let color_role = prisma_member
        .roles()?
        .first()
        .context("User has no color roles")?;

    if !member_roles
        .iter()
        .any(|n| n.id.to_string() == color_role.id.clone())
    {
        let color_role = color_role.id.clone();

        nci.member(&ctx, member.user.id)
            .await?
            .add_role(&ctx, color_role.parse::<u64>()?)
            .await?;

        role_connects.push(role::id::equals(color_role));
    }

    if member_roles.iter().any(|r| r.id == nci::roles::OVERRIDES) {
        updates.push(user::admin::set(true));
    } else {
        updates.push(user::admin::set(false));
    }

    if member_roles.iter().any(|r| r.id == nci::roles::MEMBERS) {
        updates.push(user::verified::set(true));
    } else {
        updates.push(user::verified::set(false));
    }

    if member_roles.iter().any(|r| r.id == nci::roles::BOTS) {
        updates.push(user::bot::set(true));
    } else {
        updates.push(user::bot::set(false));
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
    let prisma = prisma::create().await?;
    let nci = ctx.cache.guild(nci::ID).context("Could not find NCI")?;

    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(id.to_string()))
        .with(user::roles::fetch(vec![role::color_role::equals(true)]))
        .exec()
        .await?
        .context("Could not find user in database")?;

    let color_role = prisma_user
        .roles()?
        .first()
        .context("User has no color role")?;

    prisma
        .user()
        .update(
            user::id::equals(id.to_string()),
            vec![user::removed::set(true), user::roles::set(vec![])],
        )
        .exec()
        .await?;

    // this will send a roledelete event, handled in logging/role.rs
    nci.delete_role(&ctx, color_role.id.parse::<u64>()?).await?;

    Ok(())
}
