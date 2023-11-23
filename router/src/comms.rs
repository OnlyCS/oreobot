use crate::error::*;
use crate::request::Request;
use futures::future::BoxFuture;
use oreo_prelude::*;
use std::sync::Arc;
use std::{future::Future, marker::PhantomData};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[cfg(any(feature = "client", feature = "server", feature = "cache-server"))]
fn make_request(text: String) -> String {
    format!("LEN{},{}", text.len(), text)
}

#[cfg(any(feature = "client", feature = "server", feature = "cache-server"))]
async fn parse_message(stream: &mut TcpStream) -> Result<String, RouterError> {
    let mut reader = BufReader::new(stream);

    let mut buf = Vec::new();
    reader.read_until(b","[0], &mut buf).await?;

    let msg_len = String::from_utf8(buf)?
        .trim_start_matches("LEN")
        .trim_end_matches(",")
        .parse::<usize>()?;

    let mut buf = vec![0; msg_len];
    reader.read_exact(&mut buf).await?;

    let msg = String::from_utf8(buf)?;

    Ok(msg)
}

#[cfg(feature = "client")]
pub struct Client<Req>
where
    Req: Request,
{
    _request_type: PhantomData<Req>,
    stream: TcpStream,
}

#[cfg(feature = "client")]
impl<R> Client<R>
where
    R: Request,
{
    pub async fn new() -> Result<Self, RouterError> {
        let port = R::port();

        let stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;

        Ok(Self {
            _request_type: std::marker::PhantomData,
            stream,
        })
    }

    pub async fn send(&mut self, request: R) -> Result<R::Response, RouterError> {
        let request = make_request(serde_json::to_string(&request)?);
        self.stream.write_all(request.as_bytes()).await?;

        let response_str = parse_message(&mut self.stream).await?;
        let response = serde_json::from_str(&response_str)?;

        Ok(response)
    }
}

#[cfg(feature = "server")]
pub struct Server<Req, Callback, CallbackFut>
where
    Req: Request,
    Callback: Fn(Req) -> CallbackFut + Send + Sync + Clone + Copy + 'static,
    CallbackFut: Future<Output = Req::Response> + Send + 'static,
{
    _request_type: PhantomData<Req>,
    stream: TcpListener,
    callback: Callback,
}

#[cfg(feature = "server")]
impl<Req, Callback, CallbackFut> Server<Req, Callback, CallbackFut>
where
    Req: Request,
    Callback: Fn(Req) -> CallbackFut + Send + Sync + Clone + Copy + 'static,
    CallbackFut: Future<Output = Req::Response> + Send + 'static,
{
    pub async fn new(callback: Callback) -> Result<Self, RouterError> {
        let port = Req::port();

        let stream = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

        Ok(Self {
            _request_type: PhantomData,
            stream,
            callback,
        })
    }

    pub async fn listen(&mut self) -> Result<(), RouterError> {
        info!("Listening on port {}", Req::port());

        loop {
            let incoming = self.stream.accept().await?;
            let callback = self.callback;

            tokio::spawn(async move {
                let (mut stream, _) = incoming;

                let message_str = parse_message(&mut stream).await.unwrap();
                let message = serde_json::from_str(&message_str).unwrap();

                let response = callback(message).await;
                let response_str = make_request(serde_json::to_string(&response).unwrap());

                stream.write_all(response_str.as_bytes()).await.unwrap();
            });
        }
    }

    pub async fn listen_on_thread(mut self) {
        tokio::spawn(async move {
            self.listen().await.unwrap();
        });
    }
}

#[cfg(feature = "cache-server")]
pub struct CacheServer<Cache, Req, Callback>
where
    Req: Request,
    Callback: for<'a> Fn(Req, &'a mut Cache) -> BoxFuture<'a, Req::Response>
        + Send
        + Sync
        + Clone
        + Copy
        + 'static,
    Cache: Send + 'static,
{
    _request_type: PhantomData<Req>,
    stream: TcpListener,
    callback: Callback,
    cache: Arc<Mutex<Cache>>,
}

#[cfg(feature = "cache-server")]
impl<Cache, Req, Callback> CacheServer<Cache, Req, Callback>
where
    Req: Request,
    Callback: for<'a> Fn(Req, &'a mut Cache) -> BoxFuture<'a, Req::Response>
        + Send
        + Sync
        + Clone
        + Copy
        + 'static,
    Cache: Send + 'static,
{
    pub async fn new(cache: Cache, callback: Callback) -> Result<Self, RouterError> {
        let port = Req::port();
        let stream = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

        Ok(Self {
            _request_type: PhantomData,
            stream,
            callback,
            cache: Arc::new(Mutex::new(cache)),
        })
    }

    pub async fn listen(&mut self) -> Result<(), RouterError> {
        info!("Listening on port {}", Req::port());

        loop {
            let incoming = self.stream.accept().await?;
            let callback = self.callback;
            let cache_arc = Arc::clone(&self.cache);

            let mut cache = cache_arc.lock().await;

            let (mut stream, _) = incoming;

            let message_str = parse_message(&mut stream).await.unwrap();
            let message = serde_json::from_str(&message_str).unwrap();

            let response = callback(message, &mut cache).await;
            let response_str = make_request(serde_json::to_string(&response).unwrap());

            stream.write_all(response_str.as_bytes()).await.unwrap();
        }
    }
}
