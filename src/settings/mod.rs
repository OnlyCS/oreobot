use crate::prelude::*;
use std::any::TypeId;

#[async_trait]
pub trait Setting {
    type Data: for<'de> Deserialize<'de> + Serialize + poise::SlashArgument + Send + Sync + 'static;

    async fn default_value() -> Result<Self::Data>;
    async fn on_change(_key: Option<String>) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub data: HashMap<(TypeId, Option<String>), String>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub async fn get<T>(&self) -> Option<T::Data>
    where
        T: Setting + 'static,
    {
        let data = self
            .data
            .get(&(TypeId::of::<T>(), None))
            .map(|data| serde_json::from_str::<T::Data>(data).unwrap())
            .unwrap_or(T::default_value().await.ok()?);

        Some(data)
    }

    pub async fn get_keyed<T, S>(&self, key: S) -> Option<T::Data>
    where
        T: Setting + 'static,
        S: ToString,
    {
        let data = self
            .data
            .get(&(TypeId::of::<T>(), Some(key.to_string())))
            .map(|data| serde_json::from_str::<T::Data>(data).unwrap())
            .unwrap_or(T::default_value().await.ok()?);

        Some(data)
    }

    pub async fn set<T>(&mut self, data: T::Data) -> Result<()>
    where
        T: Setting + 'static,
    {
        self.data
            .insert((TypeId::of::<T>(), None), serde_json::to_string(&data)?);

        async_non_blocking!({ T::on_change(None).await.unwrap() });

        Ok(())
    }

    pub async fn set_keyed<T, S>(&mut self, data: T::Data, key: String) -> Result<()>
    where
        T: Setting + 'static,
    {
        self.data.insert(
            (TypeId::of::<T>(), Some(key.clone())),
            serde_json::to_string(&data)?,
        );

        async_non_blocking!({ T::on_change(Some(key)).await.unwrap() });

        Ok(())
    }
}
