use crate::prelude::*;

#[poise::command(slash_command, subcommands("add", "remove", "edit"))]
pub async fn custom(_: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

/// Add a custom role. Admins only.
#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
async fn add(
    ctx: Context<'_>,
    name: String,
    color: Color,
    add_to: Vec<Member>,
) -> Result<(), CommandError> {
    // create role
    let role = serenity::EditRole::default()
        .name(&name)
        .colour(color)
        .position(2);

    let role = ctx
        .http()
        .create_role(nci::ID, &role, Some("Oreo2: command: /role custom add"))
        .await?;

    // add role to members
    for member in add_to {
        ctx.http()
            .add_member_role(
                nci::ID,
                member.user.id,
                role.id,
                Some("Oreo2: command: /role custom add"),
            )
            .await?;
    }

    // send reply
    let embed = embed::default(EmbedStatus::Success)
        .title("Oreo2 | Custom Role | Add")
        .description(format!(
            "Successfully added role {}",
            mention::create(role.id, MentionType::Role)
        ))
        .fields(vec![("Name", name, true), ("Color", color.to_hex(), true)]);

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}

/// Remove a custom role. Admins only.
#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
async fn remove(ctx: Context<'_>, mut role: Role) -> Result<(), CommandError> {
    role.delete(&ctx).await?;

    let embed = embed::default(EmbedStatus::Success)
        .title("Oreo2 | Custom Role | Remove")
        .description(format!(
            "Successfully removed role {}",
            mention::create(role.id, MentionType::Role)
        ));

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}

/// Edit a custom role. Admins only.
#[poise::command(slash_command, required_permissions = "MANAGE_ROLES")]
async fn edit(
    ctx: Context<'_>,
    role: Role,
    name: Option<String>,
    color: Option<Color>,
    add_to: Vec<Member>,
    remove_from: Vec<Member>,
) -> Result<(), CommandError> {
    // edit role
    let mut edit = serenity::EditRole::default();

    if let Some(name) = name.as_ref() {
        edit = edit.name(name);
    }

    if let Some(color) = color.as_ref() {
        edit = edit.colour(*color);
    }

    let role = ctx
        .http()
        .edit_role(
            nci::ID,
            role.id,
            &edit,
            Some("Oreo2: command: /role custom edit"),
        )
        .await?;

    // add role to members
    for member in add_to {
        ctx.http()
            .add_member_role(
                nci::ID,
                member.user.id,
                role.id,
                Some("Oreo2: command: /role custom edit"),
            )
            .await?;
    }

    // remove role from members
    for member in remove_from {
        ctx.http()
            .remove_member_role(
                nci::ID,
                member.user.id,
                role.id,
                Some("Oreo2: command: /role custom edit"),
            )
            .await?;
    }

    // send reply
    let embed = embed::default(EmbedStatus::Success)
        .title("Oreo2 | Custom Role | Edit")
        .description(format!(
            "Successfully edited role {}",
            mention::create(role.id, MentionType::Role)
        ))
        .fields(vec![
            ("Name", role.name, true),
            ("Color", Color::from(role.colour).to_hex(), true),
        ]);

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}
