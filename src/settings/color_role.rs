use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleColor;

#[async_trait]
impl UserSetting for RoleColor {
    type Value = Color;

    async fn default_value(user: serenity::UserId) -> Result<Self::Value> {
        let prisma = prisma::create().await?;

        let color = prisma
            .role()
            .find_first(vec![role::color_role::equals(true)])
            .with(role::users::fetch(vec![user::id::equals(user.to_string())]))
            .exec()
            .await?
            .map(|n| n.color)
            .context("No color role found")?;

        Color::from_hex(color)
    }

    async fn on_change(
        ctx: &serenity::Context,
        value: Self::Value,
        user: serenity::UserId,
    ) -> Result<()> {
        let role = ctx
            .cache
            .member(nci::ID, user)
            .context("Could not find this user")?
            .roles
            .into_iter()
            .filter(|r| {
                vec![nci::roles::OVERRIDES, nci::roles::MEMBERS, nci::roles::BOTS].contains(r)
            })
            .next()
            .context("User has no color role")?;

        ctx.cache
            .guild(nci::ID)
            .context("NCI not found in cache")?
            .edit_role(&ctx, role, |edit| edit.colour(value.into()))
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleName;

#[async_trait]
impl UserSetting for RoleName {
    type Value = String;

    async fn default_value(user: serenity::UserId) -> Result<Self::Value> {
        let prisma = prisma::create().await?;

        let name = prisma
            .role()
            .find_first(vec![role::color_role::equals(true)])
            .with(role::users::fetch(vec![user::id::equals(user.to_string())]))
            .exec()
            .await?
            .map(|n| n.name)
            .context("No color role found")?;

        Ok(name)
    }

    async fn on_change(
        ctx: &serenity::Context,
        value: Self::Value,
        user: serenity::UserId,
    ) -> Result<()> {
        let role = ctx
            .cache
            .member(nci::ID, user)
            .context("Could not find this user")?
            .roles
            .into_iter()
            .filter(|r| {
                vec![nci::roles::OVERRIDES, nci::roles::MEMBERS, nci::roles::BOTS].contains(r)
            })
            .next()
            .context("User has no color role")?;

        ctx.cache
            .guild(nci::ID)
            .context("NCI not found in cache")?
            .edit_role(&ctx, role, |edit| edit.name(value))
            .await?;

        Ok(())
    }
}
