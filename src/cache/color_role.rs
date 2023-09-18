use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleColor;

#[async_trait]
impl cache::CacheItem for RoleColor {
    type Value = HashMap<serenity::UserId, Color>;
    type UpdateValue = (serenity::UserId, Color);

    async fn default_value() -> Result<Self::Value, AnyError> {
        let prisma = prisma::create().await?;

        let color = prisma
            .role()
            .find_many(vec![role::color_role::equals(true)])
            .with(role::users::fetch(vec![]))
            .exec()
            .await?
            .into_iter()
            .map(|n| (n.users().unwrap()[0].id.to_string(), n.color))
            .map(|(uid, color)| {
                (
                    serenity::UserId(u64::from_str(&uid).unwrap()),
                    Color::from_hex(color).unwrap(),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(color)
    }

    async fn update(
        ctx: &serenity::Context,
        current_value: &mut Self::Value,
        value: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        let (user_id, color) = value;

        let role_id = ctx
            .cache
            .member(nci::ID, user_id)
            .make_error(anyhow!("Could not find this user"))?
            .roles
            .into_iter()
            .filter(|r| {
                !vec![nci::roles::OVERRIDES, nci::roles::MEMBERS, nci::roles::BOTS].contains(r)
            })
            .next()
            .make_error(anyhow!("User {} has no color role", user_id))?;

        let role = ctx
            .cache
            .role(nci::ID, role_id)
            .make_error(anyhow!("Could not find role {} in cache", role_id))?;

        role.edit(&ctx, |r| r.colour(color.into())).await?;

        current_value.insert(user_id, color);

        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleName;

#[async_trait]
impl cache::CacheItem for RoleName {
    type Value = HashMap<serenity::UserId, String>;
    type UpdateValue = (serenity::UserId, String);

    async fn default_value() -> Result<Self::Value, AnyError> {
        let prisma = prisma::create().await?;

        let name = prisma
            .role()
            .find_many(vec![role::color_role::equals(true)])
            .with(role::users::fetch(vec![]))
            .exec()
            .await?
            .into_iter()
            .map(|n| (n.users().unwrap()[0].id.to_string(), n.name))
            .map(|(uid, name)| (serenity::UserId(u64::from_str(&uid).unwrap()), name))
            .collect::<HashMap<_, _>>();

        Ok(name)
    }

    async fn update(
        ctx: &serenity::Context,
        current_value: &mut Self::Value,
        value: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        let (user_id, name) = value;

        let role_id = ctx
            .cache
            .member(nci::ID, user_id)
            .make_error(anyhow!("Could not find this user"))?
            .roles
            .into_iter()
            .filter(|r| {
                !vec![nci::roles::OVERRIDES, nci::roles::MEMBERS, nci::roles::BOTS].contains(r)
            })
            .next()
            .make_error(anyhow!("User {} has no color role", user_id))?;

        let role = ctx
            .cache
            .role(nci::ID, role_id)
            .make_error(anyhow!("Could not find role {} in cache", role_id))?;

        role.edit(&ctx, |r| r.name(&name)).await?;

        current_value.insert(user_id, name);

        Ok(())
    }
}
