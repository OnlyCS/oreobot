pub mod color_role;

use crate::prelude::*;
use std::{any::TypeId, fmt::Display};

#[async_trait]
pub trait GuildSetting: Send + Sized + 'static {
    type Value: for<'de> Deserialize<'de>
        + Serialize
        + serenity::ArgumentConvert
        + Send
        + Sync
        + Clone
        + Display
        + 'static;

    async fn default_value() -> Result<Self::Value, AnyError>;
    async fn on_change(_ctx: &serenity::Context, _to: Self::Value) -> Result<(), AnyError> {
        Ok(())
    }
}

#[async_trait]
pub trait UserSetting: Send + Sized + 'static {
    type Value: for<'de> Deserialize<'de>
        + Serialize
        + serenity::ArgumentConvert
        + Send
        + Sync
        + Display
        + Clone
        + 'static;

    async fn default_value(user: serenity::UserId) -> Result<Self::Value, AnyError>;
    async fn on_change(
        _ctx: &serenity::Context,
        _to: Self::Value,
        _user: serenity::UserId,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub guild: HashMap<TypeId, String>,
    pub user: HashMap<(TypeId, serenity::UserId), String>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            guild: HashMap::new(),
            user: HashMap::new(),
        }
    }

    pub async fn get_guild<S>(&mut self) -> Result<S::Value, SettingsError>
    where
        S: GuildSetting,
    {
        let item = self.guild.get(&TypeId::of::<S>());

        if let Some(to_deser) = item {
            Ok(serde_json::from_str(to_deser)?)
        } else {
            let default =
                S::default_value()
                    .await
                    .make_error(SettingsError::DefaultValueFailed(
                        std::any::type_name::<S>().into(),
                    ))?;

            self.guild
                .insert(TypeId::of::<S>(), serde_json::to_string(&default)?);

            Ok(default)
        }
    }

    pub async fn set_guild<S>(
        &mut self,
        ctx: serenity::Context,
        value: S::Value,
    ) -> Result<(), SettingsError>
    where
        S: GuildSetting,
    {
        self.guild
            .insert(TypeId::of::<S>(), serde_json::to_string(&value)?);

        async_non_blocking!({
            S::on_change(&ctx, value)
                .await
                .make_error(SettingsError::OnChangeFailed(
                    std::any::type_name::<S>().to_string(),
                ))
                .unwrap();
        });

        Ok(())
    }

    pub async fn get_user<S>(&mut self, user: serenity::UserId) -> Result<S::Value, SettingsError>
    where
        S: UserSetting,
    {
        let item = self.user.get(&(TypeId::of::<S>(), user));

        if let Some(to_deser) = item {
            Ok(serde_json::from_str(to_deser)?)
        } else {
            let default =
                S::default_value(user)
                    .await
                    .make_error(SettingsError::DefaultValueFailed(
                        std::any::type_name::<S>().into(),
                    ))?;

            self.user
                .insert((TypeId::of::<S>(), user), serde_json::to_string(&default)?);

            Ok(default)
        }
    }

    pub async fn set_user<S>(
        &mut self,
        ctx: serenity::Context,
        value: S::Value,
        user: serenity::UserId,
    ) -> Result<(), SettingsError>
    where
        S: UserSetting,
    {
        self.user
            .insert((TypeId::of::<S>(), user), serde_json::to_string(&value)?);

        async_non_blocking!({
            S::on_change(&ctx, value, user)
                .await
                .make_error(SettingsError::OnChangeFailed(
                    std::any::type_name::<S>().to_string(),
                ))
                .unwrap();
        });

        Ok(())
    }
}

pub mod all {
    pub use super::color_role::RoleColor;
    pub use super::color_role::RoleName;
}
