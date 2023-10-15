use crate::prelude::*;

pub async fn create(category: serenity::GuildChannel) -> Result<(), CategoryLogError> {
    let prisma = prisma::create().await?;

    prisma
        .channel_category()
        .create(category.id, category.name, vec![])
        .exec()
        .await?;

    Ok(())
}

pub async fn update(category: serenity::GuildChannel) -> Result<(), CategoryLogError> {
    let prisma = prisma::create().await?;

    prisma
        .channel_category()
        .update(
            channel_category::id::equals(category.id),
            vec![channel_category::name::set(category.name)],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(category_id: serenity::ChannelId) -> Result<(), CategoryLogError> {
    let prisma = prisma::create().await?;

    let category = prisma
        .channel_category()
        .update(
            channel_category::id::equals(category_id),
            vec![channel_category::deleted::set(true)],
        )
        .with(channel_category::channels::fetch(vec![
            channel::category_id::equals(Some(category_id.into())),
        ]))
        .exec()
        .await?;

    let disconnect_channels = category
        .channels()?
        .into_iter()
        .map(|n| &n.id)
        .copied()
        .map(|n| channel::id::equals(n))
        .collect_vec();

    prisma
        .channel()
        .update_many(disconnect_channels, vec![channel::category::disconnect()])
        .exec()
        .await?;

    Ok(())
}

pub async fn get(
    category_id: serenity::ChannelId,
) -> Result<prisma::data::ChannelCategoryData, CategoryLogError> {
    let prisma = prisma::create().await?;

    let category = prisma
        .channel_category()
        .find_unique(channel_category::id::equals(category_id))
        .with(channel_category::channels::fetch(vec![]))
        .exec()
        .await?
        .make_error(CategoryLogError::NotFound(category_id))?;

    Ok(category)
}
