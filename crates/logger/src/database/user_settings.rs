use crate::prelude::*;
use std::collections::HashMap;

pub async fn create(
    user_id: serenity::UserId,
    settings: UserSettings,
) -> Result<(), UserSettingsLogError> {
    let prisma = prisma::create().await?;

    prisma
        .user_settings_data()
        .create(
            user::id::equals(user_id),
            vec![user_settings_data::pin_confirm::set(settings.pin_confirm)],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn read(user_id: serenity::UserId) -> Result<UserSettings, UserSettingsLogError> {
    let prisma = prisma::create().await?;

    let settings = prisma
        .user_settings_data()
        .find_unique(user_settings_data::user_id::equals(user_id))
        .exec()
        .await?;

    let settings = match settings.map(|n| n.into()) {
        Some(n) => n,
        None => {
            create(user_id, UserSettings::default()).await?;
            UserSettings::default()
        }
    };

    Ok(settings)
}

pub async fn update(
    user_id: serenity::UserId,
    update: UpdateUserSettings,
) -> Result<(), UserSettingsLogError> {
    let prisma = prisma::create().await?;

    let mut settings = read(user_id).await?;
    settings.update_from(update);

    prisma
        .user_settings_data()
        .update(
            user_settings_data::user_id::equals(user_id),
            vec![user_settings_data::pin_confirm::set(settings.pin_confirm)],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn all() -> Result<HashMap<serenity::UserId, UserSettings>, UserSettingsLogError> {
    let prisma = prisma::create().await?;

    let settings = prisma
        .user_settings_data()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| (n.user_id, n.into()))
        .map(|(uid, stg)| (serenity::UserId::from(uid as u64), stg))
        .collect();

    Ok(settings)
}
