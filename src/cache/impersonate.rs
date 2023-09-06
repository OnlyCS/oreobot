use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Impersonation;

#[async_trait]
impl UserCache for Impersonation {
    type Value = Option<serenity::UserId>;

    async fn default_value(_: serenity::UserId) -> Result<Self::Value, AnyError> {
        Ok(None)
    }
}
