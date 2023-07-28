use crate::prelude::*;

pub async fn create(category: &serenity::ChannelCategory, prisma: &PrismaClient) -> Result<()> {
    prisma
        .channel_category()
        .create(category.id.to_string(), category.clone().name, vec![])
        .exec()
        .await?;

    Ok(())
}

pub async fn update(category: serenity::ChannelCategory, prisma: &PrismaClient) -> Result<()> {
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

pub async fn delete(category: serenity::ChannelId, prisma: &PrismaClient) -> Result<()> {
    let category = prisma
        .channel_category()
        .delete(channel_category::id::equals(category.to_string()))
        .with(channel_category::channels::fetch(vec![
            channel::category_id::equals(Some(category.to_string())),
        ]))
        .exec()
        .await?;

    let mut where_param = vec![];

    for prisma_channel in category.channels()? {
        where_param.push(channel::id::equals(prisma_channel.id.clone()));
    }

    prisma
        .channel()
        .update_many(where_param, vec![channel::category::disconnect()])
        .exec()
        .await
        .unwrap();

    Ok(())
}
