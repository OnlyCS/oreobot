use crate::prelude::*;
use std::collections::HashMap;

pub async fn log_check_error(role: impl Into<u64>) -> Result<(), RoleLogError> {
    let role = serenity::RoleId::new(role.into());

    if nci::roles::in_blacklist(role) {
        bail!(RoleLogError::Blacklisted(role));
    }

    // custom roles handled seperately
    if prisma::create()
        .await?
        .logless_roles()
        .find_unique(logless_roles::id::equals(role))
        .exec()
        .await?
        .is_some()
    {
        bail!(RoleLogError::CustomRole(role));
    }

    Ok(())
}

/// Returns false if log_check_error returns an error
pub async fn log_check(role: impl Into<u64>) -> bool {
    log_check_error(role).await.is_ok()
}

pub async fn create(role: serenity::Role) -> Result<(), RoleLogError> {
    log_check_error(role.id).await?;

    let prisma = prisma::create().await?;

    prisma
        .role()
        .create(
            role.id,
            &role.name,
            Color::from(role.colour).to_raw_hex(),
            vec![role::color_role::set(nci::roles::is_color_role(role.id))],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn set_blacklisted(role_id: serenity::RoleId) -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;

    prisma
        .role()
        .delete(role::id::equals(role_id))
        .exec()
        .await?;

    prisma
        .logless_roles()
        .create(role_id, vec![])
        .exec()
        .await?;

    Ok(())
}

pub async fn read(role_id: serenity::RoleId) -> Result<prisma::data::RoleData, RoleLogError> {
    log_check_error(role_id).await?;

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
    log_check_error(role.id).await?;

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
    // custom roles are handled seperately
    log_check_error(role_id).await?;

    let prisma = prisma::create().await?;
    let prisma_role = read(role_id).await?;

    // if role is a color role if the user exists
    if nci::roles::is_color_role(role_id)
        && let Some(user) = prisma_role.users()?.first()
        && let uid = serenity::UserId::new(user.id as u64)
        && let BotResponse::UserExistsOk(true) = bot.send(BotRequest::UserExists(uid)).await?
        && let BotResponse::CreateRoleOk(role) = bot.send(BotRequest::CreateColorRole(uid)).await?
    {
        // delete the old role
        prisma
            .role()
            .delete(role::id::equals(prisma_role.id))
            .exec()
            .await?;

        // update the new role
        prisma
            .role()
            .update(role::id::equals(role.id), vec![role::color_role::set(true)])
            .exec()
            .await?;
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
