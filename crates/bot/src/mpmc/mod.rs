pub mod event;

use crate::prelude::*;

pub struct MpmcData {
    event: serenity::FullEvent,
    ctx: serenity::Context,
    data: Data,
}

type Reciever = async_channel::Receiver<MpmcData>;
type Sender = async_channel::Sender<MpmcData>;

static mut RECIEVER: Option<Reciever> = None;
static mut SENDER: Option<Sender> = None;

fn _rcopy() -> Option<Reciever> {
    let consumer = unsafe { RECIEVER.as_ref()? };
    let consumer = consumer.clone();
    Some(consumer)
}

fn _scopy() -> Option<Sender> {
    let sender = unsafe { SENDER.as_ref()? };
    let sender = sender.clone();
    Some(sender)
}

pub fn setup() {
    let (s, r) = async_channel::unbounded();
    unsafe { RECIEVER = Some(r) };
    unsafe { SENDER = Some(s) };
}

pub async fn send(event: MpmcData) -> Result<(), async_channel::SendError<MpmcData>> {
    let sender = _scopy().unwrap();
    sender.send(event).await
}

pub fn on<Callback, Fut>(f: Callback)
where
    Callback: Fn(serenity::Context, serenity::FullEvent, Data) -> Fut + Send + 'static,
    Fut: Future<Output = Result<(), EventError>> + Send,
{
    let recv = _rcopy().unwrap();
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
