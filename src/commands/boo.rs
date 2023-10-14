use crate::prelude::*;

#[poise::command(context_menu_command = "Boo (for Helen)")]
async fn boo(ctx: Context<'_>, message: serenity::Message) -> Result<(), CommandError> {
  message.reply(&ctx, "wooooooow. this is the most unfunny joke i have ever had the displeasure of reading. try again later.").await?;
  Ok(())
}
