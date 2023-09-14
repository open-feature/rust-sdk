use lazy_static::lazy_static;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    provider::{FeatureProvider, ProviderMetadata},
    Client,
};

use super::{
    global_evaluation_context::GlobalEvaluationContext, provider_registry::ProviderRegistry,
};

lazy_static! {
    /// The singleton instance of [`OpenFeature`] struct.
    /// The client should always use this instance to access OpenFeature APIs.
    pub static ref SINGLETON: RwLock<OpenFeature> = RwLock::new(OpenFeature::default());
}

/// THE struct of the OpenFeature API.
/// Access it via the [`SINGLETON`] instance.
#[derive(Default)]
pub struct OpenFeature {
    evaluation_context: GlobalEvaluationContext,

    pub provider_registry: ProviderRegistry,
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
        self.provider_registry.set_default(provider).await;
    }

    /// Bind the given `provider` to the corresponding `name`.
    pub async fn set_named_provider<T: FeatureProvider>(&mut self, name: &str, provider: T) {
        self.provider_registry.set_named(name, provider).await;
    }

    /// Return the metadata of default (unnamed) provider.
    pub async fn provider_metadata(&self) -> ProviderMetadata {
        self.provider_registry.get_default().await.get().metadata()
    }

    /// Return the metadata of named provider (a provider bound to clients with this name).
    pub async fn named_provider_metadata(&self, name: &str) -> Option<ProviderMetadata> {
        match self.provider_registry.get_named(name).await {
            Some(provider) => Some(provider.get().metadata()),
            None => None,
        }
    }

    /// Create a new client with default name.
    pub fn get_client(&self) -> Client {
        Client::new(
            String::default(),
            self.evaluation_context.clone(),
            self.provider_registry.clone(),
        )
    }

    /// Create a new client with specific `name`.
    /// It will use the provider bound to this name, if any.
    pub fn get_named_client(&self, name: &str) -> Client {
        Client::new(
            name.to_string(),
            self.evaluation_context.clone(),
            self.provider_registry.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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

    #[tokio::test]
    #[spec(
        number = "1.1.2.2",
        text = "The provider mutator function MUST invoke the initialize function on the newly registered provider before using it to resolve flag values."
    )]
    async fn set_provider_invoke_initialize() {
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().once().returning(|_| ());

        let mut api = OpenFeature::default();
        api.set_provider(provider).await;
    }

    #[tokio::test]
    #[spec(
        number = "1.1.3",
        text = "The API MUST provide a function to bind a given provider to one or more client names. If the client-name already has a bound provider, it is overwritten with the new mapping."
    )]
    async fn set_named_provider() {
        let mut api = OpenFeature::default();
        api.set_named_provider("test", NoOpProvider::default())
            .await;

        // Ensure the No-op provider is used.
        let client = api.get_named_client("test");
        assert_eq!(client.get_int_value("", 10, None).await, 10);

        // Bind FixedValueProvider to the same name.
        api.set_named_provider("test", FixedValueProvider::builder().int_value(30).build())
            .await;

        // Ensure the FixedValueProvider is used for existing clients.
        assert_eq!(client.get_int_value("", 10, None).await, 30);

        // Create a new client and ensure FixedValueProvideris used.
        let new_client = api.get_named_client("test");
        assert_eq!(new_client.get_int_value("", 10, None).await, 30);
    }
}
