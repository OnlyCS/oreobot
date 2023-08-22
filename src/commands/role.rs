use crate::prelude::*;

#[poise::command(slash_command)]
pub async fn role(
    ctx: Context<'_>,
    #[description = "Change the role name"] name: Option<String>,
    #[description = "Use (r,g,b) or #hex. Words (ie. \"red\") NOT accepted"] color: Option<String>,
    #[description = "The user's role to change -- admin only!"] member: Option<serenity::Member>,
) -> Result<()> {
    let loading = Loading::new(&ctx, "Connecting to the database...").await?;

    let prisma = prisma::create().await?;

    if member.is_some() && !is_admin::user(&prisma, ctx.author()).await? {
        loading
            .last(&ctx, {
                let mut embed = embed::default(&ctx, embed::EmbedStatus::Error);

                embed.title("Error");
                embed.description("You must be an admin to manage other user's roles");
                embed
            })
            .await?;

        return Ok(());
    }

    let mut fields = vec![];

    let role_id = {
        let member_id = if let Some(member) = member {
            member.user.id.to_string()
        } else {
            ctx.author().id.to_string()
        };

        let roles = prisma
            .role()
            .find_first(vec![
                role::color_role::equals(true),
                role::users::some(vec![user::id::equals(member_id)]),
            ])
            .exec()
            .await?;

        match roles {
            Some(n) => Ok(n.id),
            None => {
                let serenity = ctx.serenity_context().clone();

                // don't block the current thread
                tokio::spawn(async move {
                    logging::ready::on_ready(serenity).await.unwrap();
                });

                Err(anyhow!("Warning: The user or role could not be found. The database is being reindexed. Please try again shortly."))
            }
        }
    }?;

    if let Some(name) = name {
        loading.update(&ctx, "Updating role name...").await?;

        ctx.guild()
            .context("This command can only be used in a guild")?
            .edit_role(&ctx, role_id.parse::<u64>()?, |role| {
                role.name(name.clone());
                role
            })
            .await?;

        fields.push(("Name", name));
    }

    if let Some(color) = color {
        loading.update(&ctx, "Updating role color...").await?;

        let color_struct = {
            if color.starts_with("(") {
                let color = color.trim_start_matches("(").trim_end_matches(")");
                let number_strs = color.split(",");

                let numbers = number_strs
                    .map(|n| n.trim())
                    .map(|n| n.parse::<u8>())
                    .collect::<Vec<_>>();

                if numbers.len() != 3 {
                    bail!("Invalid color format");
                }

                serenity::Color::from((
                    numbers[0].clone()?,
                    numbers[1].clone()?,
                    numbers[2].clone()?,
                ))
            } else if color.starts_with("#") {
                colors::hex_to_color(color.trim_start_matches("#"))?
            } else {
                bail!("Invalid color format");
            }
        };

        ctx.guild()
            .context("This command can only be used in a guild")?
            .edit_role(&ctx, role_id.parse::<u64>()?, |role| {
                role.colour(color_struct.0.into());
                role
            })
            .await?;

        fields.push(("Color", color));
    }

    let mention = mention::create(role_id, mention::MentionType::Role);

    loading
        .last(&ctx, {
            let mut embed = embed::default(&ctx, embed::EmbedStatus::Sucess);

            embed.title("Role");

            if fields.is_empty() {
                embed.description(format!("No changes were made to role {}", mention));
            } else {
                embed.description(format!(
                    "Updated role {} with the following values:\n",
                    mention
                ));

                for (name, value) in fields {
                    embed.field(name, value, true);
                }
            };

            embed
        })
        .await?;

    Ok(())
}
