use crate::prelude::*;

pub(super) struct Emitter<T>
where
    T: Clone + Send + Sync + fmt::Debug,
{
    callbacks:
        Vec<Arc<dyn Fn(T) -> BoxFuture<'static, Result<(), EventError>> + 'static + Send + Sync>>,
}

impl<T> Emitter<T>
where
    T: Clone + Send + Sync + fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn on(
        &mut self,
        callback: impl Fn(T) -> BoxFuture<'static, Result<(), EventError>>
            + 'static
            + Send
            + Sync
            + Copy,
    ) {
        self.callbacks.push(Arc::new(callback));
    }

    pub async fn send(&mut self, ctx: serenity::Context, event: T) -> Result<(), EventError> {
        let event_cl = event.clone();

        let results = future::join_all(
            self.callbacks
                .par_iter()
                .map(move |f| f(event.clone()))
                .collect::<Vec<_>>(),
        )
        .await;

        let handles = results
            .into_par_iter()
            .filter_map(move |result| match result {
                Ok(_) => None,
                Err(err) => match err {
                    EventError::UnwantedEvent => {
                        debug!(
                            "event callback: unwanted event: {:?}",
                            debug_truncated(&event_cl)
                        );

                        None
                    }
                    _ => {
                        error!("event callback: {err}");
                        Some(crate::error::handle(
                            ctx.clone(),
                            err.into(),
                            nci::channels::LOGGING,
                        ))
                    }
                },
            })
            .collect::<Vec<_>>();

        let results = future::join_all(handles).await;

        for result in results {
            result?;
        }

        Ok(())
    }
}
