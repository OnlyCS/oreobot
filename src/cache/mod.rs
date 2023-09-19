pub mod color_role;
pub mod custom_role;
pub mod impersonate;
pub mod newsinchat;

use crate::prelude::*;
use std::any::TypeId;

#[async_trait]
pub trait CacheItem: Send + Sized + 'static {
    type Value: for<'de> Deserialize<'de> + Serialize + Send + Sync + Clone + 'static;
    type UpdateValue;

    async fn default_value() -> Result<Self::Value, AnyError>;

    async fn update(
        ctx: &serenity::Context,
        current_value: &mut Self::Value,
        to: Self::UpdateValue,
    ) -> Result<(), AnyError>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cache {
    pub items: HashMap<TypeId, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub async fn get<S>(&mut self) -> Result<S::Value, CacheError>
    where
        S: CacheItem,
    {
        let item = self.items.get(&TypeId::of::<S>());

        if let Some(to_deser) = item {
            Ok(serde_json::from_str(to_deser)?)
        } else {
            let default = S::default_value()
                .await
                .make_error(CacheError::DefaultValueFailed(
                    std::any::type_name::<S>().into(),
                ))?;

            self.items
                .insert(TypeId::of::<S>(), serde_json::to_string(&default)?);

            Ok(default)
        }
    }

    pub async fn update<S>(
        &mut self,
        ctx: serenity::Context,
        value: S::UpdateValue,
    ) -> Result<(), CacheError>
    where
        S: CacheItem,
    {
        let mut old_value = self.get::<S>().await?;

        S::update(&ctx, &mut old_value, value)
            .await
            .make_error(CacheError::UpdateFailed(String::from(
                std::any::type_name::<S::Value>(),
            )))?;

        self.items
            .insert(TypeId::of::<S>(), serde_json::to_string(&old_value)?);

        Ok(())
    }
}

pub mod all {
    pub use super::color_role::RoleColor;
    pub use super::color_role::RoleName;
    pub use super::custom_role::CustomRole;
    pub use super::impersonate::Impersonation;
    pub use super::newsinchat::NewsInChat;
}
