use super::common::*;
use crate::prelude::*;

pub struct Server<Meta, CallbackFut>
where
    Meta: ServerMetadata,
    CallbackFut: Future<Output = Result<Meta::Response, Meta::Error>> + Send + 'static,
{
    listener: TcpListener,
    callback: fn(Meta::Request) -> CallbackFut,
}

impl<Meta, CallbackFut> Server<Meta, CallbackFut>
where
    Meta: ServerMetadata,
    CallbackFut: Future<Output = Result<Meta::Response, Meta::Error>> + Send + 'static,
{
    pub async fn new(
        callback: fn(Meta::Request) -> CallbackFut,
    ) -> Result<Self, RouterError<Meta>> {
        let listener = TcpListener::bind(format!("{}:{}", Meta::HOST, Meta::PORT)).await?;

        Ok(Self { listener, callback })
    }

    fn accept_loop(&self, mut stream: TcpStream) -> Result<(), RouterError<Meta>> {
        debug!("Got new connection from: {}", stream.peer_addr()?);

        let callback = self.callback;

        tokio::spawn(async move {
            let reader = BufReader::new(stream.clone());
            let mut lines = reader.lines();

            while let Some(line) = lines.next().await {
                let line = match line {
                    Ok(line) => line,
                    Err(err) => {
                        error!("Error reading line: {}", err);
                        continue;
                    }
                };

                let req = serde_json::from_str(&line).unwrap();
                let res = serde_json::to_string(&((callback)(req).await)).unwrap();

                send(&mut stream, &res).await.unwrap();

                debug!(
                    "Completed server transaction: {{\n\trequest: {},\n\tresponse: {}\n}}",
                    string_truncated_dbg(line),
                    string_truncated_dbg(res)
                )
            }
        });

        Ok(())
    }

    pub async fn listen(&mut self) -> Result<!, RouterError<Meta>> {
        info!("Listening on {}:{}", Meta::HOST, Meta::PORT);

        while let Some(stream) = self.listener.incoming().next().await {
            let stream = stream?;
            self.accept_loop(stream)?;
        }

        panic!("Server stopped listening");
    }

    pub async fn listen_on_thread(mut self) {
        tokio::spawn(async move {
            self.listen().await.unwrap();
        });
    }
}
