use crate::prelude::*;

pub async fn send(stream: &mut TcpStream, msg: String) -> Result<(), tokio::io::Error> {
    debug!("Sending message: {}", debug_truncated(&msg));

    stream.write_all(format!("{msg}\n").as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
