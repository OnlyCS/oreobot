use crate::prelude::*;

pub async fn api_ping(ctx: &Context<'_>) -> Option<std::time::Duration> {
    let framework = ctx.framework();
    let shard_manager = framework.shard_manager.lock().await;
    let runners = shard_manager.runners.lock().await;
    let shard_id = serenity::ShardId(ctx.serenity_context().shard_id);
    let runner = runners.get(&shard_id)?;
    let api_latency = runner.latency?;

    Some(api_latency)
}
