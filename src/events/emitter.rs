use crate::prelude::*;

use futures::{future::BoxFuture, Future, FutureExt};
use std::{any::TypeId, collections::HashMap};

type Output = Result<()>;
type AsyncOutput = BoxFuture<'static, Output>;

pub struct Listener {
    callback: Arc<dyn Fn(Vec<u8>, serenity::Context) -> Output + Send + Sync>,
    filter: Option<Arc<dyn Fn(Vec<u8>) -> bool + Send + Sync + 'static>>,
}

pub struct AsyncListener {
    callback: Arc<dyn Fn(Vec<u8>, serenity::Context) -> AsyncOutput + Send + Sync>,
    filter: Option<Arc<dyn Fn(Vec<u8>) -> bool + Send + Sync + 'static>>,
}

pub struct EventEmitter {
    listeners: HashMap<TypeId, Vec<Listener>>,
    async_listeners: HashMap<TypeId, Vec<AsyncListener>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            async_listeners: HashMap::new(),
        }
    }

    pub async fn emit<Event>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        argument: Event::Argument,
        context: &serenity::Context,
    ) -> Result<()>
    where
        Event: EmitterEvent,
    {
        if let Some(listeners) = self.listeners.get_mut(&TypeId::of::<Event>()) {
            let bytes: Vec<u8> = serde_json::to_vec(&argument)?;

            for listener in listeners.iter_mut() {
                let bytes = bytes.clone();
                let callback = Arc::clone(&listener.callback);
                let context = context.clone();

                if let Some(filter) = &listener.filter {
                    if !filter(bytes.clone()) {
                        continue;
                    }
                }

                thread::spawn(move || {
                    callback(bytes, context.clone()).unwrap();
                });
            }
        }

        if let Some(listeners) = self.async_listeners.get_mut(&TypeId::of::<Event>()) {
            let bytes: Vec<u8> = serde_json::to_vec(&argument)?;

            for listener in listeners.iter_mut() {
                let bytes = bytes.clone();
                let callback = Arc::clone(&listener.callback);
                let context = context.clone();

                if let Some(filter) = &listener.filter {
                    if !filter(bytes.clone()) {
                        continue;
                    }
                }

                tokio::spawn(async move {
                    callback(bytes, context.clone()).await.unwrap();
                })
                .await?;
            }
        }

        Ok(())
    }

    pub fn on<Event, Callback>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        callback: Callback,
    ) where
        Event: EmitterEvent,
        Callback:
            Fn(Event::Argument, serenity::Context) -> Result<()> + Send + Sync + 'static + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            callback(serde_json::from_slice(&bytes)?, ctx)
        };

        let listener = Listener {
            callback: Arc::new(parsed_callback),
            filter: None,
        };

        match self.listeners.get_mut(&TypeId::of::<Event>()) {
            Some(callbacks) => {
                callbacks.push(listener);
            }
            None => {
                self.listeners.insert(TypeId::of::<Event>(), vec![listener]);
            }
        }
    }

    pub fn on_filter<Event, Callback, Filter>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        callback: Callback,
        filter: Filter,
    ) where
        Event: EmitterEvent,
        Callback: Fn(Event::Argument, serenity::Context) -> Result<()> + Send + Sync + 'static,
        Filter: Fn(Event::Argument) -> bool + Send + Sync + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            callback(serde_json::from_slice(&bytes)?, ctx)
        };

        let parsed_filter = move |bytes: Vec<u8>| filter(serde_json::from_slice(&bytes).unwrap());

        let listener = Listener {
            callback: Arc::new(parsed_callback),
            filter: Some(Arc::new(parsed_filter)),
        };

        match self.listeners.get_mut(&TypeId::of::<Event>()) {
            Some(callbacks) => {
                callbacks.push(listener);
            }
            None => {
                self.listeners.insert(TypeId::of::<Event>(), vec![listener]);
            }
        }
    }

    pub fn on_async<Event, Callback, Fut>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        callback: Callback,
    ) where
        Event: EmitterEvent,
        Callback: Fn(Event::Argument, serenity::Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            callback(serde_json::from_slice(&bytes).unwrap(), ctx).boxed()
        };

        let listener = AsyncListener {
            callback: Arc::new(parsed_callback),
            filter: None,
        };

        match self.async_listeners.get_mut(&TypeId::of::<Event>()) {
            Some(async_callbacks) => {
                async_callbacks.push(listener);
            }
            None => {
                self.async_listeners
                    .insert(TypeId::of::<Event>(), vec![listener]);
            }
        }
    }

    pub fn on_async_filter<Event, Callback, Fut, Filter>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        callback: Callback,
        filter: Filter,
    ) where
        Event: EmitterEvent,
        Callback: Fn(Event::Argument, serenity::Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
        Filter: Fn(Event::Argument) -> bool + Send + Sync + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            callback(serde_json::from_slice(&bytes).unwrap(), ctx).boxed()
        };

        let parsed_filter = move |bytes: Vec<u8>| filter(serde_json::from_slice(&bytes).unwrap());

        let listener = AsyncListener {
            callback: Arc::new(parsed_callback),
            filter: Some(Arc::new(parsed_filter)),
        };

        match self.async_listeners.get_mut(&TypeId::of::<Event>()) {
            Some(async_callbacks) => {
                async_callbacks.push(listener);
            }
            None => {
                self.async_listeners
                    .insert(TypeId::of::<Event>(), vec![listener]);
            }
        }
    }
}

pub trait EmitterEvent: Send + Sync + 'static {
    type Argument: Serialize + for<'a> Deserialize<'a>;
}

pub struct EventEmitterTypeKey;
impl serenity::TypeMapKey for EventEmitterTypeKey {
    type Value = Shared<EventEmitter>;
}
