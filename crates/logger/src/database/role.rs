use crate::prelude::*;
use std::collections::HashMap;

/// assuming the role is not in the database...
pub async fn infer_kind(role: &serenity::Role) -> Result<RoleType, RoleLogError> {
    let prisma = prisma::create().await?;

    let kind = if role.tags.premium_subscriber {
        RoleType::Booster
    } else if role.name == "@everyone" || role.position == 0 {
        RoleType::Everyone
    } else if role.id == nci::roles::MEMBERS {
        RoleType::MemberRole
    } else if role.id == nci::roles::BOTS {
        RoleType::BotRole
    } else if role.id == nci::roles::OVERRIDES {
        RoleType::AdminRole
    } else if role.position as usize
        >= 1 + prisma
            .role()
            .find_many(vec![role::kind::equals(RoleType::CustomRole)])
            .exec()
            .await?
            .len()
    {
        RoleType::ColorRole
    } else {
        RoleType::CustomRole
    };

    Ok(kind)
}

pub async fn create(role: serenity::Role) -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;

    prisma
        .role()
        .create(
            role.id,
            &role.name,
            Color::from(role.colour).to_raw_hex(),
            infer_kind(&role).await?,
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn read(role_id: serenity::RoleId) -> Result<prisma::data::RoleData, RoleLogError> {
    let prisma = prisma::create().await?;

    let role = prisma
        .role()
        .find_unique(role::id::equals(role_id))
        .with(role::users::fetch(vec![]))
        .exec()
        .await?
        .make_error(RoleLogError::RoleNotFound(role_id))?;

    Ok(role)
}

pub async fn update(role: serenity::Role) -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;

    prisma
        .role()
        .update(
            role::id::equals(role.id),
            vec![
                role::name::set(role.name),
                role::color::set(Color::database(role.colour)),
            ],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(
    role_id: serenity::RoleId,
    bot: &mut Client<BotServer>,
) -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;
    let prisma_role = read(role_id).await?;

    // if role is a color role if the user exists
    if prisma_role.kind == RoleType::ColorRole
        && let Some(user) = prisma_role.users()?.first()
        && let uid = serenity::UserId::new(user.id as u64)
        && let BotResponse::UserExistsOk(true) = bot.send(BotRequest::UserExists(uid)).await?
        && let BotResponse::CreateRoleOk(_) = bot.send(BotRequest::CreateColorRole(uid)).await?
    {
        // delete the old role
        prisma
            .role()
            .delete(role::id::equals(prisma_role.id))
            .exec()
            .await?;

        // the new role should have been created in role::create()
        return Ok(());
    }

    // update the role in db to set removed
    prisma
        .role()
        .update(
            role::id::equals(prisma_role.id),
            vec![role::deleted::set(true)],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn all() -> Result<HashMap<serenity::RoleId, prisma::data::RoleData>, RoleLogError> {
    let prisma = prisma::create().await?;

    let roles = prisma
        .role()
        .find_many(vec![])
        .with(role::users::fetch(vec![]))
        .exec()
        .await?
        .into_iter()
        .map(|data| (data.id, data))
        .map(|(id, data)| (serenity::RoleId::new(id as u64), data))
        .collect();

    Ok(roles)
}
