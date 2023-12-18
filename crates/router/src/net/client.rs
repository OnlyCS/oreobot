use super::common::*;
use crate::prelude::*;

pub struct Client<Meta>
where
    Meta: ServerMetadata,
{
    _metadata: PhantomData<Meta>,
    stream: TcpStream,
}

impl<Meta> Client<Meta>
where
    Meta: ServerMetadata,
{
    pub async fn new() -> Result<Self, RouterError<Meta>> {
        let mut client = Self {
            _metadata: PhantomData,
            stream: TcpStream::connect(format!("{}:{}", Meta::HOST, Meta::PORT)).await?,
        };

        client.check_ready().await?;

        Ok(client)
    }

    pub async unsafe fn send_unchecked(
        &mut self,
        request: Meta::Request,
    ) -> Result<Meta::Response, RouterError<Meta>> {
        send(&mut self.stream, serde_json::to_string(&request)?).await?;

        let reader = BufReader::new(&mut self.stream);
        let mut lines = reader.lines();

        let line = match lines.next().await {
            Some(line) => line?,
            None => bail!(RouterError::InvalidResponse),
        };

        let response: Result<Meta::Response, Meta::Error> = serde_json::from_str(&line)?;

        response.map_err(|err| RouterError::ServerError(err))
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
