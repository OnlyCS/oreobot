use crate::prelude::*;

pub async fn create(category: serenity::ChannelCategory) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .channel_category()
        .create(category.id.to_string(), category.clone().name, vec![])
        .exec()
        .await?;

    Ok(())
}

pub async fn update(category: serenity::ChannelCategory) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .channel_category()
        .update(
            channel_category::id::equals(category.id.to_string()),
            vec![channel_category::name::set(category.name)],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(category: serenity::ChannelId) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    let category = prisma
        .channel_category()
        .update(
            channel_category::id::equals(category.to_string()),
            vec![channel_category::deleted::set(true)],
        )
        .with(channel_category::channels::fetch(vec![
            channel::category_id::equals(Some(category.to_string())),
        ]))
        .exec()
        .await?;

    let disconnect_channels = category
        .channels()?
        .into_iter()
        .map(|n| &n.id)
        .cloned()
        .map(|n| channel::id::equals(n))
        .collect_vec();

    prisma
        .channel()
        .update_many(disconnect_channels, vec![channel::category::disconnect()])
        .exec()
        .await?;

    Ok(())
}
