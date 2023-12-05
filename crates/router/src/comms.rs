use crate::prelude::*;

fn make_request(text: String) -> String {
    format!("{},{}", text.len(), text)
}

async fn parse_message<Meta: ServerMetadata>(
    stream: &mut TcpStream,
) -> Result<String, RouterError<Meta>> {
    let mut reader = BufReader::new(stream);

    let mut buf = Vec::new();
    reader.read_until(b',', &mut buf).await?;

    let msg_len = String::from_utf8(buf)?
        .trim_end_matches(",")
        .parse::<usize>()?;

    let mut buf = vec![0; msg_len];
    reader.read_exact(&mut buf).await?;

    let msg = String::from_utf8(buf)?;

    Ok(msg)
}

#[cfg(feature = "client")]
pub struct Client<Meta>
where
    Meta: ServerMetadata,
{
    _metadata: PhantomData<Meta>,
    stream: TcpStream,
}

#[cfg(feature = "client")]
impl<Meta> Client<Meta>
where
    Meta: ServerMetadata,
{
    pub async fn new() -> Result<Self, RouterError<Meta>> {
        let stream = TcpStream::connect(format!("{}:{}", Meta::HOST, Meta::PORT)).await?;

        Ok(Self {
            _metadata: std::marker::PhantomData,
            stream,
        })
    }

    pub async unsafe fn send_unchecked(
        &mut self,
        request: Meta::Request,
    ) -> Result<Meta::Response, RouterError<Meta>> {
        let request = make_request(serde_json::to_string(&request)?);
        self.stream.write_all(request.as_bytes()).await?;

        let response_str = parse_message(&mut self.stream).await?;
        let response: Result<Meta::Response, Meta::Error> = serde_json::from_str(&response_str)?;

        match response {
            Ok(response) => Ok(response),
            Err(err) => Err(RouterError::ServerError(err)),
        }
    }

    pub async fn send(
        &mut self,
        request: Meta::Request,
    ) -> Result<Meta::Response, RouterError<Meta>> {
        let true = self.check_ready().await? else {
            bail!(RouterError::ServerNotReady)
        };

        unsafe { self.send_unchecked(request).await }
    }

    async fn check_ready(&mut self) -> Result<bool, RouterError<Meta>> {
        match unsafe { self.send_unchecked(Meta::READY_REQUEST).await } {
            Ok(response) => Ok({
                if response == Meta::READY_TRUE {
                    true
                } else if response == Meta::READY_FALSE {
                    false
                } else {
                    panic!("Invalid response from server")
                }
            }),
            Err(err) => Err(err),
        }
    }
}

#[cfg(feature = "server")]
pub struct Server<Meta, Callback, CallbackFut>
where
    Meta: ServerMetadata,
    Callback: Fn(Meta::Request) -> CallbackFut + Send + Sync + Clone + Copy + 'static,
    CallbackFut: Future<Output = Result<Meta::Response, Meta::Error>> + Send + 'static,
{
    _metadata: PhantomData<Meta>,
    stream: TcpListener,
    callback: Callback,
}

#[cfg(feature = "server")]
impl<Meta, Callback, CallbackFut> Server<Meta, Callback, CallbackFut>
where
    Meta: ServerMetadata,
    Callback: Fn(Meta::Request) -> CallbackFut + Send + Sync + Clone + Copy + 'static,
    CallbackFut: Future<Output = Result<Meta::Response, Meta::Error>> + Send + 'static,
{
    pub async fn new(callback: Callback) -> Result<Self, RouterError<Meta>> {
        let stream = TcpListener::bind(format!("{}:{}", Meta::HOST, Meta::PORT)).await?;

        Ok(Self {
            _metadata: PhantomData,
            stream,
            callback,
        })
    }

    pub async fn listen(&mut self) -> Result<!, RouterError<Meta>> {
        info!("Listening on {}:{}", Meta::HOST, Meta::PORT);

        loop {
            let incoming = self.stream.accept().await?;
            let callback = self.callback;

            let (mut stream, _) = incoming;

            let message_str = parse_message::<Meta>(&mut stream).await?;
            let message = serde_json::from_str(&message_str)?;

            let response = callback(message)
                .await
                .map_err(|err| RouterError::<Meta>::ServerError(err))?;

            let response_str = make_request(serde_json::to_string(&response).unwrap());

            stream.write_all(response_str.as_bytes()).await.unwrap();
        }
    }

    pub async fn listen_on_thread(mut self) {
        tokio::spawn(async move {
            self.listen().await.unwrap();
        });
    }
}

#[cfg(feature = "cache-server")]
pub struct PersistServer<Cache, Meta, Callback>
where
    Meta: ServerMetadata,
    Callback: for<'a> Fn(
            Meta::Request,
            &'a mut Cache,
        ) -> BoxFuture<'a, Result<Meta::Response, Meta::Error>>
        + Send
        + Copy
        + 'static,
    Cache: Send + 'static,
{
    _metadata: PhantomData<Meta>,
    stream: TcpListener,
    callback: Callback,
    cache: Arc<Mutex<Cache>>,
}

#[cfg(feature = "cache-server")]
impl<Cache, Meta, Callback> PersistServer<Cache, Meta, Callback>
where
    Meta: ServerMetadata,
    Callback: for<'a> Fn(
            Meta::Request,
            &'a mut Cache,
        ) -> BoxFuture<'a, Result<Meta::Response, Meta::Error>>
        + Send
        + Copy
        + 'static,
    Cache: Send + 'static,
{
    pub async fn new(cache: Cache, callback: Callback) -> Result<Self, RouterError<Meta>> {
        let stream = TcpListener::bind(format!("{}:{}", Meta::HOST, Meta::PORT)).await?;

        Ok(Self {
            _metadata: PhantomData,
            stream,
            callback,
            cache: Arc::new(Mutex::new(cache)),
        })
    }

    pub async fn listen(&mut self) -> Result<!, RouterError<Meta>> {
        info!("Listening on port {}:{}", Meta::HOST, Meta::PORT);

        loop {
            let incoming = self.stream.accept().await?;
            let callback = self.callback;
            let cache_arc = Arc::clone(&self.cache);

            let mut cache = cache_arc.lock().await;

            let (mut stream, _) = incoming;

            let message_str = parse_message::<Meta>(&mut stream).await?;
            let message = serde_json::from_str(&message_str)?;

            let response = callback(message, &mut cache).await;
            let response_str = make_request(serde_json::to_string(&response)?);

            stream.write_all(response_str.as_bytes()).await?;
        }
    }

    pub async fn listen_on_thread(mut self) {
        tokio::spawn(async move {
            self.listen().await.unwrap();
        });
    }
}
