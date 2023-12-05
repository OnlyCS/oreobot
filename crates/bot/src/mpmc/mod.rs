pub type Data = String;
pub type Reciever = async_channel::Receiver<Data>;
pub type Sender = async_channel::Sender<Data>;

static mut CONSUMER_CL: Option<Reciever> = None;

fn _copy() -> Option<Reciever> {
    let consumer = unsafe { CONSUMER_CL.as_ref().unwrap() };
    let consumer = consumer.clone();
    Some(consumer)
}

pub fn setup() -> Sender {
    let (s, r) = async_channel::unbounded();
    unsafe { CONSUMER_CL = Some(r) };

    s
}

pub fn reciever() -> Reciever {
    _copy().unwrap()
}
