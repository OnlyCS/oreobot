use crate::prelude::*;

#[poise::command(slash_command, subcommands("edit", "color_role", "add"))]
pub async fn role(_: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn color_role(
    ctx: Context<'_>,
    #[description = "Change the role name"] name: Option<String>,
    #[description = "Change the role color"] color: Option<Color>,
    #[description = "For this user (admin)"] user: Option<serenity::Member>,
) -> Result<(), CommandError> {
    let mut loading = Loading::<LoadingWithInteraction>::new(
        &ctx,
        "Locking cache. Depending on how long the bot has been up, this may take a while.",
    )
    .await?;

    let prisma = prisma::create().await?;
    let data_arc = data::get_poise(&ctx);
    let mut data = data_arc.lock().await;
    let cache = &mut data.cache;

    loading.update(&ctx, "Updating your role settings").await?;

    let user = if let Some(member) = user.as_ref() {
        if is_admin::member(&prisma, &member).await? {
            &member.user
        } else {
            loading.close(&ctx).await?;

            bail!(CommandError::RuntimeError {
                title: "Admin",
                description: "You must be an admin to change other users' roles"
            });
        }
    } else {
        ctx.author()
    };

    let mut confirmation = embed::default(&ctx, EmbedStatus::Sucess);
    confirmation.title("Role > Set");
    confirmation.description("Sucessfully updated your role");

    if let Some(name) = name {
        confirmation.field("Name", &name, true);

        cache
            .update::<cache_items::RoleName>(ctx.serenity_context().clone(), user.id, name)
            .await?;
    }

    if let Some(color) = color {
        cache
            .update::<cache_items::RoleColor>(ctx.serenity_context().clone(), user.id, color)
            .await?;

        confirmation.field("Color", color.into_hex(), true);
    }

    loading.last(&ctx, confirmation).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "The role to manage"] role: serenity::Role,
    #[description = "Change name"] name: Option<String>,
    #[description = "Change color"] color: Option<Color>,
    #[description = "Add to user"] mut add: Option<serenity::Member>,
    #[description = "Remove from user"] mut remove: Option<serenity::Member>,
) -> Result<(), CommandError> {
    let prisma = prisma::create().await?;

    // 1. check user admin
    if !is_admin::user_id(&prisma, &ctx.author().id).await? {
        return Err(CommandError::RuntimeError {
            title: "Admin",
            description: "This is an admin-only command",
        });
    }

    // 2. fetch role
    let guild = ctx.guild().make_error(CommandError::NotInGuild)?;
    let role = guild.roles.get(&role.id).unwrap();

    if let Some(name) = name.as_ref() {
        role.edit(&ctx, |r| r.name(name)).await?;
    }

    if let Some(color) = color.as_ref() {
        role.edit(&ctx, |r| r.colour((*color).into())).await?;
    }

    // 3. add to user
    if let Some(member) = add.as_mut() {
        member.add_role(&ctx, role).await?;
    }

    // 4. remove from user
    if let Some(member) = remove.as_mut() {
        member.remove_role(&ctx, role).await?;
    }

    // 5. send confirmation
    let mut confirmation = embed::default(&ctx, EmbedStatus::Sucess);
    confirmation.title("Role > Manage");
    confirmation.description(format!(
        "Sucessfully updated role {}",
        mention::create(role.id, MentionType::Role)
    ));

    if let Some(name) = name {
        confirmation.field("Name", &name, true);
    }

    if let Some(color) = color {
        confirmation.field("Color", color.into_hex(), true);
    }

    if let Some(member) = add.as_ref() {
        confirmation.field(
            "Added to",
            mention::create(member.user.id, MentionType::User),
            true,
        );
    }

    if let Some(member) = remove.as_ref() {
        confirmation.field(
            "Removed from",
            mention::create(member.user.id, MentionType::User),
            true,
        );
    }

    ctx.send(|m| {
        m.embeds.push(confirmation);
        m.ephemeral(true)
    })
    .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Role name"] name: String,
    #[description = "Role color"] color: Color,
    #[description = "Add to member"] mut member: serenity::Member,
) -> Result<(), CommandError> {
    let prisma = prisma::create().await?;
    let guild = ctx.guild().make_error(CommandError::NotInGuild)?;

    if !is_admin::user_id(&prisma, &ctx.author().id).await? {
        return Err(CommandError::RuntimeError {
            title: "Admin",
            description: "This is an admin-only command",
        });
    }

    let data_arc = data::get_poise(&ctx);
    let mut data = data_arc.lock().await;
    let cache = &mut data.cache;

    // role::create() is on hold, data is locked!
    let role = guild
        .create_role(&ctx, |role| {
            role.name(&name)
                .colour(color.into())
                .mentionable(false)
                .position(255)
        })
        .await?;

    cache
        .update::<cache_items::CustomRole>(ctx.serenity_context().clone(), (), role.id)
        .await?;

    member.add_role(&ctx, role.id).await?;

    let mut confirmation = embed::default(&ctx, EmbedStatus::Sucess);
    confirmation.title("Role > Create");
    confirmation.description(format!(
        "Successfully created a new role: {}",
        mention::create(role.id, MentionType::Role)
    ));

    confirmation.field("Name", name, true);
    confirmation.field("Color", color.into_hex(), true);

    ctx.send(|reply| {
        reply.embeds.push(confirmation);
        reply.ephemeral(true)
    })
    .await?;

    Ok(())
}
