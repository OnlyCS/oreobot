use crate::prelude::*;

pub async fn get_serenity(ctx: &serenity::Context) -> Shared<Data> {
    Arc::clone(ctx.data.read().await.get::<Data>().unwrap())
}

pub fn get_poise(ctx: &Context<'_>) -> Shared<Data> {
    Arc::clone(ctx.data())
}
