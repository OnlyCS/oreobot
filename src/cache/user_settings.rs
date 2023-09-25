use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[updatable]
pub struct SettingsData {
    pub pin_confirm: bool,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self { pin_confirm: true }
    }
}

pub struct UserSettings;

#[async_trait]
impl cache::CacheItem for UserSettings {
    type Value = HashMap<serenity::UserId, SettingsData>;
    type UpdateValue = SettingsDataUpdate;

    type InnerKey = serenity::UserId;
    type Get = SettingsData;

    async fn default_value() -> Result<Self::Value, AnyError> {
        let prisma = prisma::create().await?;

        let hm = prisma
            .user_settings_data()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|n| {
                (
                    serenity::UserId(u64::from_str(&n.user_id).unwrap()),
                    SettingsData {
                        pin_confirm: n.pin_confirm,
                    },
                )
            })
            .collect();

        Ok(hm)
    }

    async fn get(
        _ctx: &serenity::Context,
        key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError> {
        Ok(value.get(&key).copied().unwrap_or_default())
    }

    async fn update(
        _ctx: &serenity::Context,
        current_value: &mut Self::Value,
        key: Self::InnerKey,
        value: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        let option_data = current_value.get(&key).cloned();
        let prisma = prisma::create().await?;

        if let Some(mut data) = option_data {
            match value {
                SettingsDataUpdate::PinConfirm(pin_confirm) => {
                    data.pin_confirm = pin_confirm;
                }
            }

            prisma
                .user_settings_data()
                .update(
                    user_settings_data::user_id::equals(key.to_string()),
                    vec![user_settings_data::pin_confirm::set(data.pin_confirm)],
                )
                .exec()
                .await?;

            current_value.insert(key, data);
        } else {
            let data = match value {
                SettingsDataUpdate::PinConfirm(pin_confirm) => {
                    prisma
                        .user_settings_data()
                        .create(
                            user::id::equals(key.to_string()),
                            vec![user_settings_data::pin_confirm::set(pin_confirm)],
                        )
                        .exec()
                        .await?
                }
            };

            current_value.insert(
                key,
                SettingsData {
                    pin_confirm: data.pin_confirm,
                },
            );
        }

        Ok(())
    }
}
