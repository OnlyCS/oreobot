use crate::prelude::*;

pub async fn register() {
    mpmc::on(|ctx, event, _| async move {
        let FullEvent::ReactionAdd { add_reaction } = event else {
            bail!(EventError::UnwantedEvent)
        };

        if add_reaction.message(&ctx).await?.author.id == 784598998664085556
            && add_reaction
                .user_id
                .is_some_and(|n| n == 809111302198001724)
        {
            add_reaction.delete(&ctx).await?;
        }

        Ok(())
    })
    .await;
}
