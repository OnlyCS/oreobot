mod channel;
pub mod event;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct MpmcData {
    event: FullEvent,
    ctx: serenity::Context,
    data: Data,
}

lazy_static::lazy_static! {
    static ref EMITTER: Arc<Mutex<channel::Emitter<MpmcData>>> = Arc::new(Mutex::new(channel::Emitter::new()));
}

pub async fn send(event: MpmcData) -> Result<(), EventError> {
    let mut emitter = EMITTER.lock().await;
    emitter.send(event).await
}

pub async fn on<Fut>(f: fn(serenity::Context, FullEvent, Data) -> Fut)
where
    Fut: Future<Output = Result<(), EventError>> + Send + 'static,
{
    let mut emitter = EMITTER.lock().await;

    emitter.on(move |data| Box::pin(async move { f(data.ctx, data.event, data.data).await }));
}
