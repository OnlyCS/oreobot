use crate::prelude::*;

pub async fn send(stream: &mut TcpStream, msg: String) -> Result<(), tokio::io::Error> {
    debug!("Sending message: {}", string_truncated_dbg(&msg));

    stream.write_all(format!("{msg}\n").as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
