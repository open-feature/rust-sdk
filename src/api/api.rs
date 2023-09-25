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
    pub fn create_client(&self) -> Client {
        Client::new(
            String::default(),
            self.evaluation_context.clone(),
            self.provider_registry.clone(),
        )
    }

    /// Create a new client with specific `name`.
    /// It will use the provider bound to this name, if any.
    pub fn create_named_client(&self, name: &str) -> Client {
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
    use std::sync::Arc;

    use super::*;
    use crate::{provider::NoOpProvider, EvaluationContextFieldValue, EvaluationReason};
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
        let client = api.create_client();

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
        let provider = NoOpProvider::default();

        assert_eq!(provider.metadata().name, "No Operation - Default");

        let mut api = OpenFeature::default();
        api.set_provider(provider).await;

        assert_eq!(api.provider_metadata().await.name, "No Operation");
    }

    #[spec(
        number = "1.1.2.3",
        text = "The provider mutator function MUST invoke the shutdown function on the previously registered provider once it's no longer being used to resolve flag values."
    )]
    #[test]
    fn invoke_shutdown_on_old_provider_checked_by_type_system() {}

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
        let client = api.create_named_client("test");
        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 10);

        // Bind provider to the same name.
        api.set_named_provider("test", NoOpProvider::builder().int_value(30).build())
            .await;

        // Ensure the new provider is used for existing clients.
        assert_eq!(client.get_int_value("", None, None).await.unwrap(), 30);

        // Create a new client and ensure new provider is used.
        let new_client = api.create_named_client("test");
        assert_eq!(new_client.get_int_value("", None, None).await.unwrap(), 30);
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
        number = "1.1.6",
        text = "The API MUST provide a function for creating a client which accepts the following options:
        * name (optional): A logical string identifier for the client."
    )]
    #[tokio::test]
    async fn get_client() {
        let mut api = OpenFeature::default();
        api.set_provider(NoOpProvider::builder().int_value(100).build())
            .await;
        api.set_named_provider("test", NoOpProvider::builder().int_value(200).build())
            .await;

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

        let mut client = api.create_client();

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
    async fn example() {
        // Acquire an OpenFeature API instance.
        // Note the `await` call here because asynchronous lock is used to guarantee thread safety.
        let mut api = OpenFeature::singleton_mut().await;

        // Createa a global evaluation context and set it into the API.
        // Note that this is optional. By default it uses an empty one.
        let global_evaluation_context = EvaluationContext::default();
        api.set_evaluation_context(global_evaluation_context).await;

        // Set the default feature provider.
        // If you do not do that, [`NoOpProvider`] will be used by default.
        //
        // By default, [`NoOpProvider`] will simply return the default value of each type.
        // You can inject value you want via its builder or evaluation context. See its document
        // for more details.
        //
        // If you set a new provider after creating some clients, the existing clients will pick up
        // the new provider you just set.
        api.set_provider(NoOpProvider::default()).await;

        // Create an unnamed client.
        let client = api.create_client();

        // Createa an evaluation context.
        // It supports types mentioned in the specification.
        //
        // You have multiple ways to add a custom field.
        let evaluation_context = EvaluationContext::builder()
            .targeting_key("Targeting")
            .build()
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

        assert_eq!(
            client
                .get_bool_value("key", Some(&evaluation_context), None)
                .await
                .unwrap(),
            bool::default()
        );

        // Create a named provider and bind it.
        api.set_named_provider("named", NoOpProvider::builder().int_value(42).build())
            .await;

        // This named client will use the feature provider bound to this name.
        let client = api.create_named_client("named");

        assert_eq!(42, client.get_int_value("key", None, None).await.unwrap());

        // Let's get evaluation details.
        // Note that we will inject `300` as the int value via evaluation context.
        // It is not a feature mentioned in the standard but rather implemented for the
        // convenience.
        let result = client
            .get_int_details(
                "key",
                Some(&EvaluationContext::default().with_custom_field("Value", 300)),
                None,
            )
            .await;

        match result {
            Ok(details) => {
                assert_eq!(details.value, 300);
                assert_eq!(details.reason, Some(EvaluationReason::Static));
                assert_eq!(details.variant, Some("Static".to_string()));
                assert_eq!(details.flag_metadata.values.iter().count(), 1);
            }
            Err(error) => {
                println!(
                    "Error: {}\nMessage: {:?}\n",
                    error.code.to_string(),
                    error.message
                );
            }
        }
    }
}
