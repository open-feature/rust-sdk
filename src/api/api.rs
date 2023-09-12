use lazy_static::lazy_static;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    provider::{FeatureProvider, ProviderMetadata},
    Client, EvaluationContext,
};

use super::provider_registry::ProviderRegistry;

lazy_static! {
    /// The singleton instance of [`OpenFeature`] struct.
    /// The client should always use this instance to access OpenFeature APIs.
    pub static ref SINGLETON: RwLock<OpenFeature> = RwLock::new(OpenFeature::default());
}

/// THE struct of the OpenFeature API.
/// Access it via the [`SINGLETON`] instance.
#[derive(Default)]
pub struct OpenFeature {
    providers: ProviderRegistry,
    evaluation_context: EvaluationContext,
}

impl OpenFeature {
    /// Get the singleton of [`OpenFeature`].
    pub async fn singleton() -> RwLockReadGuard<'static, Self> {
        SINGLETON.read().await
    }

    /// Get a mutable singleton of [`OpenFeature`].
    pub async fn singleton_mut() -> RwLockWriteGuard<'static, Self> {
        SINGLETON.write().await
    }

    /// Set the default provider.
    pub async fn set_provider<T: FeatureProvider>(&mut self, provider: T) {
        self.providers.set_default(provider).await;
    }

    /// Bind the given `provider` to the corresponding `name`.
    pub async fn set_named_provider<T: FeatureProvider>(&mut self, name: &str, provider: T) {
        self.providers.set_named(name, provider).await;
    }

    /// Return the metadata of default (unnamed) provider.
    pub async fn provider_metadata(&self) -> ProviderMetadata {
        self.providers.get_default().await.get().metadata()
    }

    /// Return the metadata of named provider (a provider bound to clients with this name).
    pub async fn named_provider_metadata(&self, name: &str) -> Option<ProviderMetadata> {
        match self.providers.get_named(name).await {
            Some(provider) => Some(provider.get().metadata()),
            None => None,
        }
    }

    /// Create a new client with default name.
    pub fn get_client(&self) -> Client {
        Client::new(String::default(), self.providers.clone())
    }

    /// Create a new client with specific `name`.
    /// It will use the provider bound to this name, if any.
    pub fn get_named_client(&self, name: &str) -> Client {
        Client::new(name.to_string(), self.providers.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::*;
    use spec::spec;

    #[tokio::test]
    #[spec(
        number = "1.1.1",
        text = "The API, and any state it maintains SHOULD exist as a global singleton, even in cases wherein multiple versions of the API are present at runtime."
    )]
    async fn singleton_multi_thread() {
        let reader1 = tokio::spawn(async move {
            let _ = OpenFeature::singleton().await.provider_metadata();
        });

        let writer = tokio::spawn(async move {
            OpenFeature::singleton_mut()
                .await
                .set_provider(FixedValueProvider::default())
                .await;
        });

        let reader2 = tokio::spawn(async move {
            let _ = OpenFeature::singleton().await.provider_metadata();
        });

        let _ = (reader1.await, reader2.await, writer.await);

        assert_eq!(
            "Fixed Value",
            OpenFeature::singleton()
                .await
                .provider_metadata()
                .await
                .name
        );
    }

    #[tokio::test]
    #[spec(
        number = "1.1.2.1",
        text = "The API MUST define a provider mutator, a function to set the default provider, which accepts an API-conformant provider implementation."
    )]
    async fn set_provider() {
        let mut api = OpenFeature::default();
        let client = api.get_client();

        assert_eq!(client.get_int_value("some-key", 32, None).await, 32);

        // Set the new provider and ensure the value comes from it.
        let provider = FixedValueProvider::builder().int_value(200).build();
        api.set_provider(provider).await;

        assert_eq!(client.get_int_value("some-key", 100, None).await, 200);
    }
}
