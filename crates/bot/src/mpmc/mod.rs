pub mod event;

use crate::prelude::*;

pub struct MpmcData {
    event: serenity::FullEvent,
    ctx: serenity::Context,
    data: Data,
}

type Reciever = async_channel::Receiver<MpmcData>;
type Sender = async_channel::Sender<MpmcData>;

lazy_static::lazy_static! {
    static ref SENDER_RECIEVER: (Sender, Reciever) = async_channel::unbounded();
}

fn _copy_sender() -> Sender {
    SENDER_RECIEVER.0.clone()
}

fn _copy_reciever() -> Reciever {
    SENDER_RECIEVER.1.clone()
}

pub async fn send(event: MpmcData) -> Result<(), async_channel::SendError<MpmcData>> {
    let sender = _copy_sender();
    sender.send(event).await
}

pub fn on<Fut>(f: fn(serenity::Context, serenity::FullEvent, Data) -> Fut)
where
    Fut: Future<Output = Result<(), EventError>> + Send + 'static,
{
    let recv = _copy_reciever();
    tokio::spawn(async move {
        while let Ok(event) = recv.recv().await {
            let MpmcData { ctx, event, data } = event;

            match f(ctx, event, data).await {
                Ok(_) => {}
                Err(e) => error!("error in event callback: {e}"),
            }
        }
    });
}
