use crate::prelude::*;

pub async fn member(prisma: &PrismaClient, member: &serenity::Member) -> Result<bool, PrismaError> {
    user(prisma, &member.user).await
}

pub async fn user(prisma: &PrismaClient, user: &serenity::User) -> Result<bool, PrismaError> {
    user_id(prisma, &user.id).await
}

pub async fn user_id(
    prisma: &PrismaClient,
    user_id: &serenity::UserId,
) -> Result<bool, PrismaError> {
    let user = prisma
        .user()
        .find_unique(user::id::equals(user_id.to_string()))
        .exec()
        .await?
        .make_error(PrismaError::NotFound(format!("user with id {}", user_id)))?;

    Ok(user.admin)
}
