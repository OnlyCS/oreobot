use crate::prelude::*;

use std::collections::HashMap;

use futures::{future::BoxFuture, Future, FutureExt};
use serde_json;

pub struct Listener {
    callback: Arc<dyn Fn(Vec<u8>, serenity::Context) -> Result<()> + Send + Sync>,
    limit: Option<u32>,
}

pub struct AsyncListener {
    callback:
        Arc<dyn Fn(Vec<u8>, serenity::Context) -> BoxFuture<'static, Result<()>> + Send + Sync>,
    limit: Option<u32>,
}

pub struct EventEmitter<Event>
where
    Event: Eq + Clone + std::hash::Hash + Send + Sync,
{
    listeners: HashMap<Event, Vec<Listener>>,
    async_listeners: HashMap<Event, Vec<AsyncListener>>,
}

impl<Event> EventEmitter<Event>
where
    Event: Eq + Clone + std::hash::Hash + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            async_listeners: HashMap::new(),
        }
    }

    pub fn on<F, T>(&mut self, event: &Event, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T, serenity::Context) -> Result<()> + Send + Sync + 'static,
    {
        self._on(event, None, callback)
    }

    pub fn on_once<F, T>(&mut self, event: &Event, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T, serenity::Context) -> Result<()> + Send + Sync + 'static,
    {
        self.on_limited(event, 1, callback)
    }

    pub fn on_limited<F, T>(&mut self, event: &Event, limit: u32, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T, serenity::Context) -> Result<()> + Send + Sync + 'static,
    {
        self._on(event, Some(limit), callback)
    }

    pub fn on_async<F, T, A>(&mut self, event: &Event, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        A: Future<Output = Result<()>> + Send + 'static,
        F: Fn(T, serenity::Context) -> A + Send + Sync + 'static,
    {
        self._on_async(event, None, callback)
    }

    pub fn on_once_async<F, T, A>(&mut self, event: &Event, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        A: Future<Output = Result<()>> + Send + 'static,
        F: Fn(T, serenity::Context) -> A + Send + Sync + 'static,
    {
        self.on_limited_async(event, 1, callback)
    }

    pub fn on_limited_async<F, T, A>(&mut self, event: &Event, limit: u32, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        A: Future<Output = Result<()>> + Send + 'static,
        F: Fn(T, serenity::Context) -> A + Send + Sync + 'static,
    {
        self._on_async(event, Some(limit), callback)
    }

    pub async fn emit<T>(
        &mut self,
        event: &Event,
        value: T,
        context: &serenity::Context,
    ) -> Result<()>
    where
        T: Serialize,
    {
        if let Some(listeners) = self.listeners.get_mut(event) {
            let bytes: Vec<u8> = serde_json::to_vec(&value)?;

            let mut listeners_to_remove: Vec<usize> = Vec::new();
            for (index, listener) in listeners.iter_mut().enumerate() {
                let cloned_bytes = bytes.clone();
                let callback = Arc::clone(&listener.callback);
                let context = context.clone();

                match listener.limit {
                    None => {
                        thread::spawn(move || {
                            callback(cloned_bytes, context.clone()).unwrap();
                        });
                    }
                    Some(limit) => {
                        if limit != 0 {
                            thread::spawn(move || {
                                callback(cloned_bytes, context.clone()).unwrap();
                            });
                            listener.limit = Some(limit - 1);
                        } else {
                            listeners_to_remove.push(index);
                        }
                    }
                }
            }

            for index in listeners_to_remove.into_iter().rev() {
                listeners.remove(index);
            }
        }

        if let Some(listeners) = self.async_listeners.get_mut(event) {
            let bytes: Vec<u8> = serde_json::to_vec(&value)?;

            let mut listeners_to_remove = Vec::new();
            for (index, listener) in listeners.iter_mut().enumerate() {
                let cloned_bytes = bytes.clone();
                let callback = Arc::clone(&listener.callback);
                let context = context.clone();

                match listener.limit {
                    None => {
                        tokio::spawn(async move {
                            callback(cloned_bytes, context.clone()).await.unwrap();
                        })
                        .await?;
                    }
                    Some(limit) => {
                        if limit != 0 {
                            tokio::spawn(async move {
                                callback(cloned_bytes, context.clone()).await.unwrap();
                            })
                            .await?;
                            listener.limit = Some(limit - 1);
                        } else {
                            listeners_to_remove.push(index);
                        }
                    }
                }
            }

            for index in listeners_to_remove.into_iter().rev() {
                listeners.remove(index);
            }
        }

        Ok(())
    }

    fn _on<F, T>(&mut self, event: &Event, limit: Option<u32>, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T, serenity::Context) -> Result<()> + Send + Sync + 'static + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            let value: T = serde_json::from_slice(&bytes).unwrap();
            callback(value, ctx)
        };

        let listener = Listener {
            limit,
            callback: Arc::new(parsed_callback),
        };

        match self.listeners.get_mut(event) {
            Some(callbacks) => {
                callbacks.push(listener);
            }
            None => {
                self.listeners.insert(event.clone(), vec![listener]);
            }
        }
    }

    fn _on_async<F, T, A>(&mut self, event: &Event, limit: Option<u32>, callback: F)
    where
        for<'de> T: Deserialize<'de>,
        A: Future<Output = Result<()>> + Send + 'static,
        F: Fn(T, serenity::Context) -> A + Send + Sync + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            let value: T = serde_json::from_slice(&bytes).unwrap();
            callback(value, ctx).boxed()
        };

        let listener = AsyncListener {
            limit,
            callback: Arc::new(parsed_callback),
        };

        match self.async_listeners.get_mut(event) {
            Some(async_callbacks) => {
                async_callbacks.push(listener);
            }
            None => {
                self.async_listeners.insert(event.clone(), vec![listener]);
            }
        }
    }
}

pub struct EventEmitterTypeKey;

impl serenity::TypeMapKey for EventEmitterTypeKey {
    type Value = Shared<EventEmitter<EmitterEvent>>;
}
