use crate::prelude::*;

pub struct CustomRole;

#[async_trait]
impl cache::CacheItem for CustomRole {
    type Value = Vec<serenity::RoleId>;
    type UpdateValue = serenity::RoleId;

    type InnerKey = ();
    type Get = Vec<serenity::RoleId>;

    async fn default_value() -> Result<Self::Value, AnyError> {
        let prisma = prisma::create().await?;

        Ok(prisma
            .logless_roles()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|n| n.id)
            .map(|n| serenity::RoleId(n.parse().unwrap()))
            .collect_vec())
    }

    async fn get(
        _ctx: &serenity::Context,
        _key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError> {
        Ok(value)
    }

    async fn update(
        _ctx: &serenity::Context,
        current_value: &mut Self::Value,
        _key: Self::InnerKey,
        to: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        current_value.push(to);

        let prisma = prisma::create().await?;

        prisma
            .logless_roles()
            .create(to.to_string(), vec![])
            .exec()
            .await?;

        Ok(())
    }
}
