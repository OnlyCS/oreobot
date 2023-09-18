use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Impersonation;

#[async_trait]
impl cache::CacheItem for Impersonation {
    type Value = HashMap<serenity::UserId, Option<serenity::UserId>>;
    type UpdateValue = (serenity::UserId, Option<serenity::UserId>);

    async fn default_value() -> Result<Self::Value, AnyError> {
        Ok(HashMap::new())
    }

    async fn update(
        _ctx: &serenity::Context,
        current_value: &mut Self::Value,
        to: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        current_value.insert(to.0, to.1);

        Ok(())
    }
}
