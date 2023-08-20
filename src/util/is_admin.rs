use crate::prelude::*;

pub async fn member(prisma: &PrismaClient, member: &serenity::Member) -> Result<bool> {
    user(prisma, &member.user).await
}

pub async fn user(prisma: &PrismaClient, user: &serenity::User) -> Result<bool> {
    user_id(prisma, &user.id).await
}

pub async fn user_id(prisma: &PrismaClient, user_id: &serenity::UserId) -> Result<bool> {
    let user_id = user_id.to_string();

    let user = prisma
        .user()
        .find_unique(user::id::equals(user_id))
        .exec()
        .await?
        .context("Could not find user")?;

    Ok(user.admin)
}
