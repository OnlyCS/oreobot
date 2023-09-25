use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Impersonation;

#[async_trait]
impl cache::CacheItem for Impersonation {
    type Value = HashMap<serenity::UserId, Option<serenity::UserId>>;
    type UpdateValue = Option<serenity::UserId>;
    type InnerKey = serenity::UserId;
    type Get = Option<serenity::UserId>;

    async fn default_value() -> Result<Self::Value, AnyError> {
        Ok(HashMap::new())
    }

    async fn get(
        _ctx: &serenity::Context,
        key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError> {
        Ok(value.get(&key).copied().flatten())
    }

    async fn update(
        _ctx: &serenity::Context,
        current_value: &mut Self::Value,
        key: Self::InnerKey,
        to: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        current_value.insert(key, to);

        Ok(())
    }
}
