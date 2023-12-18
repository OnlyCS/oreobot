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

    pub async fn send(&mut self, event: T) -> Result<(), EventError> {
        let event_cl = event.clone();

        let results = future::join_all(
            self.callbacks
                .par_iter()
                .map(move |f| f(event.clone()))
                .collect::<Vec<_>>(),
        )
        .await;

        results
            .into_par_iter()
            .for_each(move |result| match result {
                Ok(_) => {}
                Err(err) => match err {
                    EventError::UnwantedEvent => {
                        debug!(
                            "event callback: unwanted event: {:?}",
                            debug_truncated(&event_cl)
                        )
                    }
                    _ => error!("event callback: {err}"),
                },
            });

        Ok(())
    }
}
