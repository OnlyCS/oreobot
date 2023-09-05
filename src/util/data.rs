use crate::prelude::*;

pub async fn get_serenity(ctx: &serenity::Context) -> Result<Shared<Data>> {
    Ok(Arc::clone(
        ctx.data
            .read()
            .await
            .get::<Data>()
            .context("Could not find data")?,
    ))
}

pub fn get_poise(ctx: &Context<'_>) -> Shared<Data> {
    Arc::clone(ctx.data())
}
