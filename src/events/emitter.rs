use crate::prelude::*;

use futures::{future::BoxFuture, Future, FutureExt};
use std::{any::TypeId, collections::HashMap};

type Output = Result<(), AnyError>;
type AsyncOutput = BoxFuture<'static, Output>;

pub struct Listener {
    callback: Arc<dyn Fn(Vec<u8>, serenity::Context) -> AsyncOutput + Send + Sync>,
    filter: Option<Arc<dyn Fn(Vec<u8>) -> bool + Send + Sync + 'static>>,
}

pub struct EventEmitter {
    listeners: HashMap<TypeId, Vec<Listener>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub async fn emit<Event>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        argument: Event::Argument,
        context: &serenity::Context,
    ) -> Result<(), EmitterError>
    where
        Event: EmitterEvent,
    {
        let Some(listeners) = self.listeners.get_mut(&TypeId::of::<Event>()) else {
            return Ok(());
        };

        let bytes: Vec<u8> = serde_json::to_vec(&argument)?;

        for listener in listeners.iter_mut() {
            let bytes = bytes.clone();

            if let Some(filter) = &listener.filter {
                if !filter(bytes.clone()) {
                    continue;
                }
            }

            let callback = listener.callback.clone();
            let context = context.clone();

            async_non_blocking!({ callback(bytes.clone(), context).await.unwrap() });
        }

        Ok(())
    }

    pub fn on<Event, Callback, Fut>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        callback: Callback,
    ) where
        Event: EmitterEvent,
        Callback: Fn(Event::Argument, serenity::Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Output> + Send + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            callback(serde_json::from_slice(&bytes).unwrap(), ctx).boxed()
        };

        let listener = Listener {
            callback: Arc::new(parsed_callback),
            filter: None,
        };

        match self.listeners.get_mut(&TypeId::of::<Event>()) {
            Some(async_callbacks) => {
                async_callbacks.push(listener);
            }
            None => {
                self.listeners.insert(TypeId::of::<Event>(), vec![listener]);
            }
        }
    }

    pub fn on_filter<Event, Callback, Fut, Filter>(
        &mut self,
        _event: Event, /* making the user specify generic argument for this looks ugly af */
        callback: Callback,
        filter: Filter,
    ) where
        Event: EmitterEvent,
        Callback: Fn(Event::Argument, serenity::Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Output> + Send + 'static,
        Filter: Fn(Event::Argument) -> bool + Send + Sync + 'static,
    {
        let parsed_callback = move |bytes: Vec<u8>, ctx: serenity::Context| {
            callback(serde_json::from_slice(&bytes).unwrap(), ctx).boxed()
        };

        let parsed_filter = move |bytes: Vec<u8>| filter(serde_json::from_slice(&bytes).unwrap());

        let listener = Listener {
            callback: Arc::new(parsed_callback),
            filter: Some(Arc::new(parsed_filter)),
        };

        match self.listeners.get_mut(&TypeId::of::<Event>()) {
            Some(async_callbacks) => {
                async_callbacks.push(listener);
            }
            None => {
                self.listeners.insert(TypeId::of::<Event>(), vec![listener]);
            }
        }
    }
}

pub trait EmitterEvent: Send + Sync + 'static {
    type Argument: Serialize + for<'a> Deserialize<'a>;
}
