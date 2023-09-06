use crate::prelude::*;

pub struct Loading<'a> {
    handle: poise::ReplyHandle<'a>,
}

impl<'a> Loading<'a> {
    pub async fn new<S>(ctx: &'a Context<'_>, msg: S) -> Result<Loading<'a>, serenity::Error>
    where
        S: ToString,
    {
        let handle = ctx
            .send(|message| {
                let mut embed = embed::default(&ctx, EmbedStatus::Warning);

                embed.title("Loading");
                embed.description(msg);

                message
                    .embed(|f| {
                        f.clone_from(&embed);
                        f
                    })
                    .ephemeral(true)
                    .components(|c| c.set_action_row(share::row(true)))
            })
            .await?;

        Ok(Self { handle })
    }

    pub async fn update<S>(&self, ctx: &'a Context<'_>, msg: S) -> Result<(), serenity::Error>
    where
        S: ToString,
    {
        self.handle
            .edit(ctx.clone(), |message| {
                let mut embed = embed::default(&ctx, EmbedStatus::Warning);

                embed.title("Loading");
                embed.description(msg);

                message
                    .embed(|f| {
                        f.clone_from(&embed);
                        f
                    })
                    .ephemeral(true)
                    .components(|c| c.set_action_row(share::row(true)))
            })
            .await?;

        Ok(())
    }

    pub async fn last(
        self,
        ctx: &'a Context<'_>,
        embed: serenity::CreateEmbed,
    ) -> Result<poise::ReplyHandle<'a>, serenity::Error> {
        self.handle
            .edit(ctx.clone(), |edit| {
                edit.embed(|e| {
                    e.clone_from(&embed);
                    e
                })
                .components(|c| c.set_action_row(share::row(false)))
                .ephemeral(true)
            })
            .await?;

        Ok(self.handle)
    }
}
