use lazy_static::lazy_static;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    provider::{FeatureProvider, ProviderMetadata},
    Client, EvaluationContext,
};

use super::{
    global_evaluation_context::GlobalEvaluationContext, provider_registry::ProviderRegistry,
};

lazy_static! {
    /// The singleton instance of [`OpenFeature`] struct.
    /// The client should always use this instance to access OpenFeature APIs.
    static ref SINGLETON: RwLock<OpenFeature> = RwLock::new(OpenFeature::default());
}

/// THE struct of the OpenFeature API.
/// Access it via the [`SINGLETON`] instance.
#[derive(Default)]
pub struct OpenFeature {
    evaluation_context: GlobalEvaluationContext,

    provider_registry: ProviderRegistry,
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

    pub async fn set_evaluation_context(&mut self, evaluation_context: EvaluationContext) {
        let mut context = self.evaluation_context.get_mut().await;

        context.targeting_key = evaluation_context.targeting_key;
        context.custom_fields = evaluation_context.custom_fields;
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
        self.provider_registry
            .get_default()
            .await
            .get()
            .metadata()
            .clone()
    }

    /// Return the metadata of named provider (a provider bound to clients with this name).
    pub async fn named_provider_metadata(&self, name: &str) -> Option<ProviderMetadata> {
        match self.provider_registry.get_named(name).await {
            Some(provider) => Some(provider.get().metadata().clone()),
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

    /// Drops all the registered providers.
    pub async fn shutdown(&mut self) {
        self.provider_registry.clear().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{provider::*, EvaluationContextFieldValue};
    use spec::spec;

    #[spec(
        number = "1.1.1",
        text = "The API, and any state it maintains SHOULD exist as a global singleton, even in cases wherein multiple versions of the API are present at runtime."
    )]
    #[tokio::test]
    async fn singleton_multi_thread() {
        let reader1 = tokio::spawn(async move {
            let _ = OpenFeature::singleton().await.provider_metadata();
        });

        let writer = tokio::spawn(async move {
            OpenFeature::singleton_mut()
                .await
                .set_provider(NoOpProvider::default())
                .await;
        });

        let reader2 = tokio::spawn(async move {
            let _ = OpenFeature::singleton().await.provider_metadata();
        });

        let _ = (reader1.await, reader2.await, writer.await);

        assert_eq!(
            "No Operation",
            OpenFeature::singleton()
                .await
                .provider_metadata()
                .await
                .name
        );
    }

    #[spec(
        number = "1.1.2.1",
        text = "The API MUST define a provider mutator, a function to set the default provider, which accepts an API-conformant provider implementation."
    )]
    #[tokio::test]
    async fn set_provider() {
        let mut api = OpenFeature::default();
        let client = api.get_client();

        assert_eq!(
            client.get_int_value("some-key", None, None).await.unwrap(),
            i64::default()
        );

        // Set the new provider and ensure the value comes from it.
        let provider = NoOpProvider::builder().int_value(200).build();
        api.set_provider(provider).await;

        assert_eq!(
            client.get_int_value("some-key", None, None).await.unwrap(),
            200
        );
    }

    #[spec(
        number = "1.1.2.2",
        text = "The provider mutator function MUST invoke the initialize function on the newly registered provider before using it to resolve flag values."
    )]
    #[tokio::test]
    async fn set_provider_invoke_initialize() {
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().once().returning(|_| ());

        let mut api = OpenFeature::default();
        api.set_provider(provider).await;
    }

    #[spec(
        number = "1.1.3",
        text = "The API MUST provide a function to bind a given provider to one or more client names. If the client-name already has a bound provider, it is overwritten with the new mapping."
    )]
    #[tokio::test]
    async fn set_named_provider() {
        let mut api = OpenFeature::default();
        api.set_named_provider("test", NoOpProvider::builder().int_value(10).build())
            .await;

        // Ensure the No-op provider is used.
        let client = api.get_named_client("test");
        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 10);

        // Bind provider to the same name.
        api.set_named_provider("test", NoOpProvider::builder().int_value(30).build())
            .await;

        // Ensure the new provider is used for existing clients.
        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 30);

        // Create a new client and ensure new provider is used.
        let new_client = api.get_named_client("test");
        assert_eq!(new_client.get_int_value("", None, None).await.unwrap(), 30);
    }

    #[spec(
        number = "1.1.4",
        text = "The API MUST provide a function to add hooks which accepts one or more API-conformant hooks, and appends them to the collection of any previously added hooks. When new hooks are added, previously added hooks are not removed."
    )]
    #[tokio::test]
    async fn add_hooks() {
        // Not implemented.
    }

    #[spec(
        number = "1.1.5",
        text = "The API MUST provide a function for retrieving the metadata field of the configured provider."
    )]
    #[tokio::test]
    async fn provider_metadata() {
        let mut api = OpenFeature::default();
        api.set_provider(NoOpProvider::default()).await;
        api.set_named_provider("test", NoOpProvider::default())
            .await;

        assert_eq!(api.provider_metadata().await.name, "No Operation");
        assert_eq!(
            api.named_provider_metadata("test").await.unwrap().name,
            "No Operation"
        );
        assert!(api.named_provider_metadata("invalid").await.is_none());
    }

    #[spec(
        number = "1.6.1",
        text = "The API MUST define a shutdown function which, when called, must call the respective shutdown function on the active provider."
    )]
    #[tokio::test]
    async fn shutdown() {
        let mut api = OpenFeature::default();
        api.set_provider(NoOpProvider::default()).await;

        api.shutdown().await;
    }

    #[spec(
        number = "3.2.1.1",
        text = "The API, Client and invocation MUST have a method for supplying evaluation context."
    )]
    #[spec(
        number = "3.2.3",
        text = "Evaluation context MUST be merged in the order: API (global; lowest precedence) -> client -> invocation -> before hooks (highest precedence), with duplicate values being overwritten."
    )]
    #[tokio::test]
    async fn evaluation_context() {
        // Ensure the value set into provider is picked up.
        let mut api = OpenFeature::default();
        api.set_provider(NoOpProvider::builder().int_value(100).build())
            .await;

        let mut client = api.get_client();

        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 100);

        // Ensure the value set into global context is picked up.
        api.set_evaluation_context(
            EvaluationContext::default()
                .with_custom_field("Value", EvaluationContextFieldValue::Int(200)),
        )
        .await;

        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 200);

        // Set another provider to the API and ensure its value is picked up.
        api.set_evaluation_context(
            EvaluationContext::default()
                .with_custom_field("Value", EvaluationContextFieldValue::Int(150)),
        )
        .await;

        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 150);

        // Ensure the value set into client context is picked up.
        client.set_evaluation_context(
            EvaluationContext::default()
                .with_custom_field("Value", EvaluationContextFieldValue::Int(300)),
        );

        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 300);

        // Ensure the value set into invocation evaluation context is picked up.
        client.set_evaluation_context(
            EvaluationContext::default()
                .with_custom_field("Value", EvaluationContextFieldValue::Int(400)),
        );

        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 400);
    }
}
