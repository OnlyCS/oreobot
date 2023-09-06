use crate::prelude::*;

pub struct WithInteraction;
pub struct WithoutInteraction;

enum LoadingMessage<'a> {
    WithInteraction(poise::ReplyHandle<'a>),
    WithoutInteraction(serenity::Message),
}

impl<'a> LoadingMessage<'a> {
    async fn update<S>(
        &mut self,
        ctx_a: Option<Context<'_>>,
        ctx_b: Option<serenity::Context>,
        update: S,
    ) -> Result<(), serenity::Error>
    where
        S: ToString,
    {
        match self {
            LoadingMessage::WithInteraction(handle) => {
                handle
                    .edit(ctx_a.unwrap().clone(), |message| {
                        let mut embed = embed::default(&ctx_a.unwrap(), EmbedStatus::Warning);

                        embed.title("Loading");
                        embed.description(update);

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
            LoadingMessage::WithoutInteraction(msg) => {
                msg.edit(&ctx_b.clone().unwrap(), |message| {
                    let mut embed = embed::default(ctx_b.as_ref().unwrap(), EmbedStatus::Warning);

                    embed.title("Loading");
                    embed.description(update);

                    message.embed(|f| {
                        f.clone_from(&embed);
                        f
                    })
                })
                .await?;

                Ok(())
            }
        }
    }

    async fn last(
        &mut self,
        ctx_a: Option<Context<'_>>,
        ctx_b: Option<serenity::Context>,
        update: serenity::CreateEmbed,
    ) -> Result<(), serenity::Error> {
        match self {
            LoadingMessage::WithInteraction(handle) => {
                handle
                    .edit(ctx_a.unwrap().clone(), |message| {
                        message
                            .embed(|f| {
                                f.clone_from(&update);
                                f
                            })
                            .ephemeral(true)
                            .components(|c| c.set_action_row(share::row(false)))
                    })
                    .await?;

                Ok(())
            }
            LoadingMessage::WithoutInteraction(msg) => {
                msg.edit(&ctx_b.clone().unwrap(), |message| {
                    message.embed(|f| {
                        f.clone_from(&update);
                        f
                    })
                })
                .await?;

                Ok(())
            }
        }
    }

    fn inner_handle(self) -> Option<poise::ReplyHandle<'a>> {
        match self {
            LoadingMessage::WithInteraction(handle) => Some(handle),
            LoadingMessage::WithoutInteraction(_) => None,
        }
    }

    fn inner_message(self) -> Option<serenity::Message> {
        match self {
            LoadingMessage::WithInteraction(_) => None,
            LoadingMessage::WithoutInteraction(msg) => Some(msg),
        }
    }

    async fn close(
        self,
        ctx_a: Option<Context<'_>>,
        ctx_b: Option<serenity::Context>,
    ) -> Result<(), serenity::Error> {
        match self {
            LoadingMessage::WithInteraction(handle) => {
                handle.delete(ctx_a.unwrap()).await?;
            }
            LoadingMessage::WithoutInteraction(msg) => {
                msg.delete(&ctx_b.unwrap()).await?;
            }
        }

        Ok(())
    }
}

pub struct Loading<'a, Type> {
    _type: std::marker::PhantomData<Type>,
    inner: LoadingMessage<'a>,
}

impl<'a> Loading<'a, WithInteraction> {
    pub async fn new<S>(
        ctx: &'a Context<'_>,
        msg: S,
    ) -> Result<Loading<'a, WithInteraction>, serenity::Error>
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

        Ok(Self {
            _type: std::marker::PhantomData,
            inner: LoadingMessage::WithInteraction(handle),
        })
    }

    pub async fn update<S>(&mut self, ctx: &'a Context<'_>, msg: S) -> Result<(), serenity::Error>
    where
        S: ToString,
    {
        self.inner.update(Some(*ctx), None, msg).await?;

        Ok(())
    }

    pub async fn last(
        mut self,
        ctx: &'a Context<'_>,
        embed: serenity::CreateEmbed,
    ) -> Result<poise::ReplyHandle<'a>, serenity::Error> {
        self.inner.last(Some(*ctx), None, embed).await?;

        Ok(self.inner.inner_handle().unwrap())
    }

    pub async fn close(self, ctx: &'a Context<'_>) -> Result<(), serenity::Error> {
        self.inner.close(Some(*ctx), None).await?;

        Ok(())
    }
}

impl<'a> Loading<'a, WithoutInteraction> {
    pub async fn new<S>(
        ctx: &serenity::Context,
        channel: serenity::ChannelId,
        msg: S,
    ) -> Result<Loading<'a, WithoutInteraction>, serenity::Error>
    where
        S: ToString,
    {
        let msg = channel
            .send_message(&ctx, |message| {
                let mut embed = embed::default(&ctx, EmbedStatus::Warning);

                embed.title("Loading");
                embed.description(msg);

                message.embed(|f| {
                    f.clone_from(&embed);
                    f
                })
            })
            .await?;

        Ok(Self {
            _type: std::marker::PhantomData,
            inner: LoadingMessage::WithoutInteraction(msg),
        })
    }

    pub async fn update<S>(
        &mut self,
        ctx: &serenity::Context,
        msg: S,
    ) -> Result<(), serenity::Error>
    where
        S: ToString,
    {
        self.inner.update(None, Some(ctx.clone()), msg).await?;

        Ok(())
    }

    pub async fn last(
        mut self,
        ctx: &serenity::Context,
        embed: serenity::CreateEmbed,
    ) -> Result<serenity::Message, serenity::Error> {
        self.inner.last(None, Some(ctx.clone()), embed).await?;

        Ok(self.inner.inner_message().unwrap())
    }

    pub async fn close(self, ctx: &serenity::Context) -> Result<(), serenity::Error> {
        self.inner.close(None, Some(ctx.clone())).await?;

        Ok(())
    }
}
