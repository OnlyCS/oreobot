use crate::prelude::*;

pub async fn send(stream: &mut TcpStream, msg: impl Into<String>) -> Result<(), tokio::io::Error> {
    stream
        .write_all(format!("{}\n", msg.into()).as_bytes())
        .await?;

    stream.flush().await?;

    Ok(())
}
