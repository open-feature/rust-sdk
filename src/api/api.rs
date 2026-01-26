use lazy_static::lazy_static;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    provider::{FeatureProvider, ProviderMetadata},
    Client, EvaluationContext, Hook, HookWrapper,
};

use super::{
    global_evaluation_context::GlobalEvaluationContext, global_hooks::GlobalHooks,
    provider_registry::ProviderRegistry,
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
    hooks: GlobalHooks,

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

    /// Set the global evaluation context.
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

    /// Add a new hook to the global list of hooks.
    pub async fn add_hook<T: Hook>(&mut self, hook: T) {
        let mut lock = self.hooks.get_mut().await;
        lock.push(HookWrapper::new(hook));
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
        self.provider_registry
            .get_named(name)
            .await
            .map(|provider| provider.get().metadata().clone())
    }

    /// Create a new client with default name.
    pub fn create_client(&self) -> Client {
        Client::new(
            String::default(),
            self.evaluation_context.clone(),
            self.hooks.clone(),
            self.provider_registry.clone(),
        )
    }

    /// Create a new client with specific `name`.
    /// It will use the provider bound to this name, if any.
    pub fn create_named_client(&self, name: &str) -> Client {
        Client::new(
            name.to_string(),
            self.evaluation_context.clone(),
            self.hooks.clone(),
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
    use std::sync::Arc;

    use super::*;
    use crate::{
        provider::{MockFeatureProvider, NoOpProvider, ResolutionDetails},
        EvaluationContextFieldValue,
    };
    use mockall::predicate;
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
            "No-op Provider",
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
        let client = api.create_client();

        assert!(client.get_int_value("some-key", None, None).await.is_err());

        // Set the new provider and ensure the value comes from it.
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});
        provider.expect_hooks().return_const(vec![]);
        provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        provider
            .expect_resolve_int_value()
            .return_const(Ok(ResolutionDetails::new(200)));

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
        provider.expect_initialize().returning(|_| {}).once();

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

        // Ensure the No-op provider is used.
        let client = api.create_named_client("test");
        assert!(client.get_int_value("", None, None).await.is_err());

        // Bind provider to the same name.
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});
        provider.expect_hooks().return_const(vec![]);
        provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        provider
            .expect_resolve_int_value()
            .return_const(Ok(ResolutionDetails::new(30)));
        api.set_named_provider("test", provider).await;

        // Ensure the new provider is used for existing clients.
        assert_eq!(client.get_int_value("", None, None).await, Ok(30));

        // Create a new client and ensure new provider is used.
        let new_client = api.create_named_client("test");
        assert_eq!(new_client.get_int_value("", None, None).await, Ok(30));
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

        assert_eq!(api.provider_metadata().await.name, "No-op Provider");
        assert_eq!(
            api.named_provider_metadata("test").await.unwrap().name,
            "No-op Provider"
        );
        assert!(api.named_provider_metadata("invalid").await.is_none());
    }

    #[spec(
        number = "1.1.6",
        text = "The API MUST provide a function for creating a client which accepts the following options:
        * name (optional): A logical string identifier for the client."
    )]
    #[tokio::test]
    async fn get_client() {
        let mut api = OpenFeature::default();

        let mut default_provider = MockFeatureProvider::new();
        default_provider.expect_initialize().returning(|_| {});
        default_provider.expect_hooks().return_const(vec![]);
        default_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        default_provider
            .expect_resolve_int_value()
            .return_const(Ok(ResolutionDetails::new(100)));

        let mut named_provider = MockFeatureProvider::new();
        named_provider.expect_initialize().returning(|_| {});
        named_provider.expect_hooks().return_const(vec![]);
        named_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        named_provider
            .expect_resolve_int_value()
            .return_const(Ok(ResolutionDetails::new(200)));

        api.set_provider(default_provider).await;
        api.set_named_provider("test", named_provider).await;

        let client = api.create_client();
        assert_eq!(client.get_int_value("key", None, None).await.unwrap(), 100);

        let client = api.create_named_client("test");
        assert_eq!(client.get_int_value("key", None, None).await.unwrap(), 200);

        let client = api.create_named_client("another");
        assert_eq!(client.get_int_value("test", None, None).await.unwrap(), 100);
    }

    #[spec(
        number = "1.1.7",
        text = "The client creation function MUST NOT throw, or otherwise abnormally terminate."
    )]
    #[test]
    fn get_client_not_throw_checked_by_type_system() {}

    #[spec(
        number = "1.1.8",
        text = "The API SHOULD provide functions to set a provider and wait for the initialize function to return or throw."
    )]
    #[tokio::test]
    async fn set_provider_should_block() {
        let mut api = OpenFeature::default();
        api.set_provider(NoOpProvider::default()).await;

        api.set_named_provider("named", NoOpProvider::default())
            .await;
    }

    #[spec(
        number = "1.6.1",
        text = "The API MUST define a shutdown function which, when called, must call the respective shutdown function on the active provider."
    )]
    #[tokio::test]
    async fn shutdown_calls_provider_shutdown() {
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});
        provider.expect_shutdown().once().returning(|| {});

        let mut api = OpenFeature::default();
        api.set_provider(provider).await;

        api.shutdown().await;
    }

    #[spec(
        number = "1.6.1",
        text = "The API MUST define a shutdown function which, when called, must call the respective shutdown function on the active provider."
    )]
    #[tokio::test]
    async fn shutdown_calls_shutdown_on_named_providers() {
        let mut default_provider = MockFeatureProvider::new();
        default_provider.expect_initialize().returning(|_| {});
        default_provider.expect_shutdown().once().returning(|| {});

        let mut named_provider = MockFeatureProvider::new();
        named_provider.expect_initialize().returning(|_| {});
        named_provider.expect_shutdown().once().returning(|| {});

        let mut api = OpenFeature::default();
        api.set_provider(default_provider).await;
        api.set_named_provider("test", named_provider).await;

        api.shutdown().await;
    }

    #[spec(
        number = "1.1.2.3",
        text = "The provider mutator function MUST invoke the shutdown function on the previously registered provider once it's no longer being used to resolve flag values."
    )]
    #[tokio::test]
    async fn set_provider_calls_shutdown_on_old_provider() {
        let mut old_provider = MockFeatureProvider::new();
        old_provider.expect_initialize().returning(|_| {});
        old_provider.expect_shutdown().once().returning(|| {});

        let mut new_provider = MockFeatureProvider::new();
        new_provider.expect_initialize().returning(|_| {});

        let mut api = OpenFeature::default();
        api.set_provider(old_provider).await;

        // When we set a new provider, the old one's shutdown should be called
        api.set_provider(new_provider).await;
    }

    #[spec(
        number = "1.1.2.3",
        text = "The provider mutator function MUST invoke the shutdown function on the previously registered provider once it's no longer being used to resolve flag values."
    )]
    #[tokio::test]
    async fn set_named_provider_calls_shutdown_on_old_provider() {
        let mut old_provider = MockFeatureProvider::new();
        old_provider.expect_initialize().returning(|_| {});
        old_provider.expect_shutdown().once().returning(|| {});

        let mut new_provider = MockFeatureProvider::new();
        new_provider.expect_initialize().returning(|_| {});

        let mut api = OpenFeature::default();
        api.set_named_provider("test", old_provider).await;

        // When we set a new provider with the same name, the old one's shutdown should be called
        api.set_named_provider("test", new_provider).await;
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
        // Setup expectations for different evaluation contexts.
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});
        provider.expect_hooks().return_const(vec![]);
        provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());

        provider
            .expect_resolve_int_value()
            .with(
                predicate::eq("flag"),
                predicate::eq(
                    EvaluationContext::default()
                        .with_targeting_key("global_targeting_key")
                        .with_custom_field("key", "global_value"),
                ),
            )
            .return_const(Ok(ResolutionDetails::new(100)));

        provider
            .expect_resolve_int_value()
            .with(
                predicate::eq("flag"),
                predicate::eq(
                    EvaluationContext::default()
                        .with_targeting_key("client_targeting_key")
                        .with_custom_field("key", "client_value"),
                ),
            )
            .return_const(Ok(ResolutionDetails::new(200)));

        provider
            .expect_resolve_int_value()
            .with(
                predicate::eq("flag"),
                predicate::eq(
                    EvaluationContext::default()
                        .with_targeting_key("invocation_targeting_key")
                        .with_custom_field("key", "invocation_value"),
                ),
            )
            .return_const(Ok(ResolutionDetails::new(300)));

        // Register the provider.
        let mut api = OpenFeature::default();
        api.set_provider(provider).await;

        // Set global client context and ensure its values are picked up.
        let global_evaluation_context = EvaluationContext::default()
            .with_targeting_key("global_targeting_key")
            .with_custom_field("key", "global_value");

        api.set_evaluation_context(global_evaluation_context).await;

        let mut client = api.create_client();

        assert_eq!(client.get_int_value("flag", None, None).await.unwrap(), 100);

        // Set client evaluation context and ensure its values overwrite the global ones.
        let client_evaluation_context = EvaluationContext::default()
            .with_targeting_key("client_targeting_key")
            .with_custom_field("key", "client_value");

        client.set_evaluation_context(client_evaluation_context);

        assert_eq!(client.get_int_value("flag", None, None).await.unwrap(), 200);

        // Use invocation level evaluation context and ensure its values are used.
        let invocation_evaluation_context = EvaluationContext::default()
            .with_targeting_key("invocation_targeting_key")
            .with_custom_field("key", "invocation_value");

        assert_eq!(
            client
                .get_int_value("flag", Some(&invocation_evaluation_context), None)
                .await
                .unwrap(),
            300
        );
    }

    #[spec(
        number = "3.2.2.1",
        text = "The API MUST have a method for setting the global evaluation context."
    )]
    #[spec(
        number = "3.2.2.2",
        text = "The Client and invocation MUST NOT have a method for supplying evaluation context."
    )]
    #[spec(
        number = "3.2.4.1",
        text = "When the global evaluation context is set, the on context changed handler MUST run."
    )]
    #[test]
    fn static_context_not_applicable() {}

    #[derive(Clone, Default, Debug)]
    struct MyStruct {}

    #[tokio::test]
    async fn extended_example() {
        // Acquire an OpenFeature API instance.
        let mut api = OpenFeature::singleton_mut().await;

        // Set the default (unnamed) provider.
        api.set_provider(NoOpProvider::default()).await;

        // Create an unnamed client.
        let client = api.create_client();

        // Create an evaluation context.
        // It supports types mentioned in the specification.
        let evaluation_context = EvaluationContext::default()
            .with_targeting_key("Targeting")
            .with_custom_field("bool_key", true)
            .with_custom_field("int_key", 100)
            .with_custom_field("float_key", 3.14)
            .with_custom_field("string_key", "Hello".to_string())
            .with_custom_field("datetime_key", time::OffsetDateTime::now_utc())
            .with_custom_field(
                "struct_key",
                EvaluationContextFieldValue::Struct(Arc::new(MyStruct::default())),
            )
            .with_custom_field("another_struct_key", Arc::new(MyStruct::default()))
            .with_custom_field(
                "yet_another_struct_key",
                EvaluationContextFieldValue::new_struct(MyStruct::default()),
            );

        // This function returns a `Result`.
        // You can process it with functions provided by std.
        let is_feature_enabled = client
            .get_bool_value("SomeFlagEnabled", Some(&evaluation_context), None)
            .await
            .unwrap_or(false);

        if is_feature_enabled {
            // Let's get evaluation details.
            let _result = client
                .get_int_details("key", Some(&evaluation_context), None)
                .await;
        }
    }
}
