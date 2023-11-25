use serde::{Deserialize, Serialize};

pub trait Request: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static {
    type Response: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    const PORT: u16;
    const HOST: &'static str;
}
