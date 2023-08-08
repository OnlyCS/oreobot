use crate::prelude::*;

pub async fn create(role: serenity::Role, ctx: serenity::Context) -> Result<()> {
    get_prisma::from_serenity_context!(prisma, ctx);

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
    get_prisma::from_serenity_context!(prisma, ctx);

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
    get_prisma::from_serenity_context!(prisma, ctx);

    prisma
        .role()
        .update(
            role::id::equals(role.to_string()),
            vec![role::deleted::set(true)],
        )
        .exec()
        .await?;

    Ok(())
}
