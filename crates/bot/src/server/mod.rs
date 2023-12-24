use crate::prelude::*;

async fn server(req: BotRequest, ctx: &serenity::Context) -> Result<BotResponse, BotServerError> {
    let response = match req {
        BotRequest::IsReady => BotResponse::Ready,
        BotRequest::AddRoleToUser(user_id, role_id) => {
            ctx.http
                .add_member_role(
                    nci::ID,
                    user_id,
                    role_id,
                    Some("Oreo2: Got request to add color role"),
                )
                .await?;

            BotResponse::UpdateOk
        }
        BotRequest::CreateColorRole(user_id) => {
            let mut logger = Client::<LoggingServer>::new().await?;
            let nci = ctx.http.get_guild(nci::ID).await?;

            let custom_role_len = {
                let LoggingResponse::AllRolesOk(roles) =
                    logger.send(LoggingRequest::RoleReadAll).await?
                else {
                    bail!(RouterError::InvalidResponse)
                };

                roles
                    .into_values()
                    .filter(|data| data.kind == RoleType::CustomRole)
                    .count() as u16
            };

            let role = nci
                .create_role(
                    &ctx,
                    serenity::EditRole::default()
                        .name("color role")
                        .position(custom_role_len + 1),
                )
                .await?;

            ctx.http
                .add_member_role(nci::ID, user_id, role.id, Some("Oreo2: Add color role"))
                .await?;

            BotResponse::CreateRoleOk(role)
        }
        BotRequest::DeleteRole(role_id) => {
            ctx.http
                .delete_role(nci::ID, role_id, Some("Oreo2: Got request to delete role"))
                .await?;

            BotResponse::UpdateOk
        }
        BotRequest::GetAllCategories => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let categories = nci
                .channels(&ctx)
                .await?
                .into_values()
                .filter(|channel| channel.kind == ChannelType::Category)
                .collect_vec();

            BotResponse::CategoriesOk(categories)
        }
        BotRequest::GetAllChannels => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let channels = nci
                .channels(&ctx)
                .await?
                .into_values()
                .filter(|channel| channel.kind != ChannelType::Category)
                .collect_vec();

            BotResponse::ChannelsOk(channels)
        }
        BotRequest::GetAllMembers => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let members = nci.members(&ctx, None, None).await?;

            BotResponse::MembersOk(members)
        }
        BotRequest::GetAllRoles => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let roles = nci.roles.into_values().collect_vec();

            BotResponse::RolesOk(roles)
        }
        BotRequest::GetRolesOfUser(user_id) => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let member = nci.member(&ctx, user_id).await?;

            BotResponse::RolesOk(
                member
                    .roles
                    .into_iter()
                    .filter_map(|n| nci.roles.get(&n))
                    .cloned()
                    .collect_vec(),
            )
        }
        BotRequest::RemoveRoleFromUser(user_id, role_id) => {
            ctx.http
                .remove_member_role(
                    nci::ID,
                    user_id,
                    role_id,
                    Some("Oreo2: Got request to remove color role"),
                )
                .await?;

            BotResponse::UpdateOk
        }
        BotRequest::UserExists(user_id) => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let member = nci.member(&ctx, user_id).await;

            BotResponse::UserExistsOk(member.is_ok())
        }
        BotRequest::GetMember(user_id) => {
            let nci = ctx.http.get_guild(nci::ID).await?;
            let member = nci.member(&ctx, user_id).await?;

            BotResponse::MemberOk(member)
        }
    };

    Ok(response)
}

async fn _run(ctx: serenity::Context) -> Result<!, RouterError<BotServer>> {
    let mut server = PersistServer::new(ctx, |req, ctx| {
        Box::pin(async move { server(req, ctx).await })
    })
    .await?;

    server.listen().await
}

pub async fn run(ctx: serenity::Context) -> Result<(), RouterError<BotServer>> {
    tokio::spawn(async move { _run(ctx).await.unwrap() });

    Ok(())
}
