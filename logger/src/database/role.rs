use crate::prelude::*;

pub fn log_check(role: serenity::RoleId) -> Result<(), RoleLogError> {
    if !nci::roles::can_log(role) {
        bail!(RoleLogError::Blacklisted(role));
    }

    // custom roles handled seperately
    if todo!("Check if role is a custom role, and if so, skip") {
        bail!(RoleLogError::CustomRole(role));
    }

    Ok(())
}

pub async fn create(role: serenity::Role) -> Result<(), RoleLogError> {
    log_check(role.id)?;

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

pub async fn update(role: serenity::Role) -> Result<(), RoleLogError> {
    log_check(role.id)?;

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

pub async fn delete(role_id: serenity::RoleId) -> Result<(), RoleLogError> {
    // custom roles are handled seperately
    log_check(role_id)?;

    let prisma = prisma::create().await?;

    let prisma_role = get(role_id).await?;

    // if role is a color role if the user exists
    if nci::roles::is_color_role(role_id) && todo!("Comms: check user has not left") {
        todo!("Comms: create role");
        todo!("Add role to user");
        todo!("Update role in database (id::equals>role, id::set>new_role");
    }

    Ok(())
}

pub async fn get(role_id: serenity::RoleId) -> Result<prisma::data::RoleData, RoleLogError> {
    log_check(role_id)?;

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
