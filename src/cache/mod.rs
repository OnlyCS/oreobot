pub mod color_role;
pub mod custom_role;
pub mod impersonate;
pub mod newsinchat;
pub mod user_settings;

use crate::prelude::*;
use std::any::TypeId;

#[async_trait]
pub trait CacheItem: Send + Sized + 'static {
    type Value: for<'de> Deserialize<'de> + Serialize + Send + Sync + Clone + 'static;
    type UpdateValue;

    type InnerKey;
    type Get;

    async fn default_value() -> Result<Self::Value, AnyError>;

    async fn get(
        ctx: &serenity::Context,
        key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError>;

    async fn update(
        ctx: &serenity::Context,
        current_value: &mut Self::Value,
        key: Self::InnerKey,
        to: Self::UpdateValue,
    ) -> Result<(), AnyError>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cache {
    items: HashMap<TypeId, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub async fn get<S>(
        &mut self,
        ctx: serenity::Context,
        inner_key: S::InnerKey,
    ) -> Result<S::Get, CacheError>
    where
        S: CacheItem,
    {
        let full_value = self._get_full_value::<S>().await?;

        Ok(S::get(&ctx, inner_key, full_value)
            .await
            .make_error(CacheError::GetFailed(
                std::any::type_name::<S::Value>().into(),
            ))?)
    }

    async fn _get_full_value<S>(&mut self) -> Result<S::Value, CacheError>
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
        key: S::InnerKey,
        value: S::UpdateValue,
    ) -> Result<(), CacheError>
    where
        S: CacheItem,
    {
        let mut full_value = self._get_full_value::<S>().await?;

        S::update(&ctx, &mut full_value, key, value)
            .await
            .make_error(CacheError::UpdateFailed(String::from(
                std::any::type_name::<S::Value>(),
            )))?;

        self.items
            .insert(TypeId::of::<S>(), serde_json::to_string(&full_value)?);

        Ok(())
    }
}

pub mod all {
    pub use super::color_role::RoleColor;
    pub use super::color_role::RoleName;
    pub use super::custom_role::CustomRole;
    pub use super::impersonate::Impersonation;
    pub use super::newsinchat::NewsInChat;
    pub use super::user_settings::UserSettings;
}
