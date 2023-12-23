use super::common::*;
use crate::prelude::*;

pub struct PersistServer<Cache, Meta>
where
    Meta: ServerMetadata,
    Cache: Send + 'static,
{
    stream: TcpListener,
    callback: for<'a> fn(
        Meta::Request,
        &'a mut Cache,
    ) -> BoxFuture<'a, Result<Meta::Response, Meta::Error>>,
    cache: Arc<Mutex<Cache>>,
}

impl<Cache, Meta> PersistServer<Cache, Meta>
where
    Meta: ServerMetadata,
    Cache: Send + 'static,
{
    pub async fn new(
        cache: Cache,
        callback: for<'a> fn(
            Meta::Request,
            &'a mut Cache,
        ) -> BoxFuture<'a, Result<Meta::Response, Meta::Error>>,
    ) -> Result<Self, RouterError<Meta>> {
        let stream = TcpListener::bind(format!("{}:{}", Meta::HOST, Meta::PORT)).await?;

        Ok(Self {
            stream,
            callback,
            cache: Arc::new(Mutex::new(cache)),
        })
    }

    fn accept_loop(&self, mut stream: TcpStream) -> Result<(), RouterError<Meta>> {
        debug!("Got new connection from: {}", stream.peer_addr()?);

        let cache = Arc::clone(&self.cache);
        let callback = self.callback;

        tokio::spawn(async move {
            let reader = BufReader::new(stream.clone());
            let mut lines = reader.lines();

            while let Some(line) = lines.next().await {
                let mut cache = cache.lock().await;

                let line = match line {
                    Ok(line) => line,
                    Err(err) => {
                        error!("Error reading line: {}", err);
                        continue;
                    }
                };

                debug!("Got request: {}", string_truncated_dbg(&line));

                let req = serde_json::from_str(&line).unwrap();
                let res = serde_json::to_string(&((callback)(req, &mut cache).await)).unwrap();
                drop(cache);

                send(&mut stream, res).await.unwrap();
            }
        });

        Ok(())
    }

    pub async fn listen(&mut self) -> Result<!, RouterError<Meta>> {
        info!("Listening on address {}:{}", Meta::HOST, Meta::PORT);

        while let Some(stream) = self.stream.incoming().next().await {
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
