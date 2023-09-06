use crate::prelude::*;

macro_rules! user_setting {
    ($fnname:ident,$stg:ident,$stgname:expr) => {
		#[poise::command(slash_command)]
        pub async fn $fnname(
            ctx: Context<'_>,
            #[description = "the value to set"] value: Option<<settings::$stg as UserCache>::Value>,
			#[description = "manage this user, admins only"] user: Option<serenity::Member>
        ) -> Result<(), CommandError> {
			let mut loading = Loading::<LoadingWithInteraction>::new(&ctx, "Locking settings.\nDepending on how long the bot has been up, this may take awhile.").await?;
			let mut data = ctx.data().lock().await;
			let cache = &mut data.cache;

			let user = match user {
				Some(u) => {
					if is_admin::user(&prisma::create().await?, ctx.author()).await? {
						u.user
					} else {
						let mut embed = embed::default(&ctx, EmbedStatus::Error);

						embed.title(format!("Settings: Needs Admin"));
						embed.description(format!("You must have admin permissions to manage other users"));

						return Ok(())
					}
				}
				None => {
					ctx.author().clone()
				}
			};

            if let Some(value) = value {
				loading.update(&ctx, format!("Updating setting {}", $stgname)).await?;
				cache.set_user::<settings::$stg>(ctx.serenity_context().clone(), value, user.id).await?;

				let mut embed = embed::default(&ctx, EmbedStatus::Sucess);

				embed.title(format!("Settings > {} > Set", $stgname));
				embed.description(format!("Sucessfully set the {} setting to `{}`", $stgname, cache.get_user::<settings::$stg>(user.id).await?));

				loading.last(&ctx, embed).await?;
			} else {
				let mut embed = embed::default(&ctx, EmbedStatus::Sucess);

				embed.title(format!("Settings > {} > Get", $stgname));
				embed.description(format!("The value of the setting is {}", cache.get_user::<settings::$stg>(user.id).await?));

				loading.last(&ctx, embed).await?;
			}

            Ok(())
        }
    };
}

macro_rules! settings_wrapper {
    ($($fnname:expr),*) => {
        #[poise::command(slash_command, subcommands($($fnname,)*))]
        pub async fn settings(_: Context<'_>) -> Result<(), CommandError> {
            Ok(())
        }
    };
}

user_setting!(role_color, RoleColor, "Role Color");
user_setting!(role_name, RoleName, "Role Name");

settings_wrapper!("role_color", "role_name");
