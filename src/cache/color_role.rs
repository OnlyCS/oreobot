use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleColor;

#[async_trait]
impl cache::CacheItem for RoleColor {
    type Value = HashMap<serenity::UserId, Color>;
    type UpdateValue = Color;

    type InnerKey = serenity::UserId;
    type Get = Color;

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

    async fn get(
        ctx: &serenity::Context,
        key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError> {
        let get = value.get(&key).copied();

        if let Some(g) = get {
            Ok(g)
        } else {
            Ok(Self::default_value()
                .await
                .map(|v| v.get(&key).copied())?
                .make_error(anyhow!("Could not find color role for user {}", key))?)
        }
    }

    async fn update(
        ctx: &serenity::Context,
        current_value: &mut Self::Value,
        key: Self::InnerKey,
        value: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        let role_id = ctx
            .cache
            .member(nci::ID, key)
            .make_error(anyhow!("Could not find this user"))?
            .roles
            .into_iter()
            .filter(|r| nci::roles::is_color_role(*r))
            .next()
            .make_error(anyhow!("User {} has no color role", key))?;

        let role = ctx
            .cache
            .role(nci::ID, role_id)
            .make_error(anyhow!("Could not find role {} in cache", role_id))?;

        role.edit(&ctx, |r| r.colour(value.into())).await?;

        current_value.insert(key, value);

        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RoleName;

#[async_trait]
impl cache::CacheItem for RoleName {
    type Value = HashMap<serenity::UserId, String>;
    type UpdateValue = String;

    type InnerKey = serenity::UserId;
    type Get = String;

    async fn default_value() -> Result<Self::Value, AnyError> {
        let prisma = prisma::create().await?;

        let name = prisma
            .role()
            .find_many(vec![role::color_role::equals(true)])
            .with(role::users::fetch(vec![]))
            .exec()
            .await?
            .into_iter()
            .filter(|n| !n.deleted)
            .map(|n| (n.users().unwrap().clone(), n.name))
            .map(|n| {
                info!("{:?}", n);
                n
            })
            .map(|(users, name)| (users[0].id.to_string(), name))
            .map(|(uid, name)| (serenity::UserId(u64::from_str(&uid).unwrap()), name))
            .collect::<HashMap<_, _>>();

        Ok(name)
    }

    async fn get(
        ctx: &serenity::Context,
        key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError> {
        let get = value.get(&key).cloned();

        if let Some(g) = get {
            Ok(g)
        } else {
            Ok(Self::default_value()
                .await
                .map(|v| v.get(&key).cloned())?
                .make_error(anyhow!("Could not find color role for user {}", key))?)
        }
    }

    async fn update(
        ctx: &serenity::Context,
        current_value: &mut Self::Value,
        key: Self::InnerKey,
        value: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        let user_id = key;
        let name = value;

        let role_id = ctx
            .cache
            .member(nci::ID, user_id)
            .make_error(anyhow!("Could not find this user"))?
            .roles
            .into_iter()
            .filter(|r| nci::roles::is_color_role(*r))
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
