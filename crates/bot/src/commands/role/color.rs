use crate::prelude::*;

/// Update the color or name of your color role.
#[poise::command(slash_command)]
pub async fn color(
    ctx: Context<'_>,
    name: Option<String>,
    color: Option<Color>,
    #[description = "Update values for someone else. Admin only."] member: Option<Member>,
) -> Result<(), CommandError> {
    let mut logger = Client::<LoggingServer>::new().await?;

    // check if member is admin
    let LoggingResponse::MemberOk(prisma::data::UserData {
        admin: is_admin, ..
    }) = logger
        .send(LoggingRequest::MemberRead(ctx.author().id))
        .await?
    else {
        bail!(RouterError::<LoggingServer>::InvalidResponse);
    };

    // needs admin to update someone else
    if member.is_some() && !is_admin {
        bail!(CommandError::AdminRequired);
    }

    // get the member to update
    let member = member
        .map(|m| m.user)
        .unwrap_or_else(|| ctx.author().clone());

    // request member data
    let LoggingResponse::MemberOk(member_data) =
        logger.send(LoggingRequest::MemberRead(member.id)).await?
    else {
        bail!(RouterError::<LoggingServer>::InvalidResponse);
    };

    // get color role of member
    let role_data = member_data
        .roles()?
        .into_iter()
        .filter(|r| r.color_role)
        .next()
        .make_error(CommandError::NoColorRole(
            member_data
                .nickname
                .as_ref()
                .unwrap_or(&member_data.username)
                .clone(),
        ))?;

    let role_id = serenity::RoleId::new(role_data.id as u64);

    // update role
    let mut new_name = &role_data.name;
    let mut new_color = Color::try_from(&role_data.color)?;
    let mut edit = serenity::EditRole::default()
        .hoist(true)
        .position(10)
        .mentionable(false);

    if let Some(color) = color {
        edit = edit.colour(color);
        new_color = color;
    }

    if let Some(name) = name.as_ref() {
        edit = edit.name(name);
        new_name = name;
    }

    ctx.http()
        .edit_role(nci::ID, role_id, &edit, Some("Oreo2: command: /role color"))
        .await?;

    // create embed and reply
    let embed = embed::default(EmbedStatus::Success)
        .title("Oreo2 | Color Role")
        .description(format!(
            "Updated color role of {}: {}",
            mention::create(member.id, MentionType::User),
            mention::create(role_id, MentionType::Role)
        ))
        .fields(vec![
            ("Name", new_name, true),
            ("Color", &new_color.to_hex(), true),
        ]);

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}
