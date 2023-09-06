use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

use crate::{
    provider::{FeatureProvider, NoOpProvider, ProviderMetadata},
    Client, EvaluationContext,
};

lazy_static! {
    /// The singleton instance of [`OpenFeature`] struct.
    /// The client should always use this instance to access OpenFeature APIs.
    pub static ref SINGLETON: RwLock<OpenFeature> = RwLock::new(OpenFeature {
        provider: Arc::new(NoOpProvider::default()),
        evaluation_context: EvaluationContext::default()
    });
}

/// THE struct of the OpenFeature API.
/// Access it via the [`SINGLETON`] instance.
pub struct OpenFeature {
    provider: Arc<dyn FeatureProvider>,
    evaluation_context: EvaluationContext,
}

impl OpenFeature {
    pub fn new<T: FeatureProvider>(provider: T, evaluation_context: EvaluationContext) -> Self {
        Self {
            provider: Arc::new(provider),
            evaluation_context,
        }
    }

    pub fn set_provider<T>(&mut self, provider: T)
    where
        T: FeatureProvider,
    {
        self.provider = Arc::new(provider);
    }

    pub fn provider_metadata(&self) -> &ProviderMetadata {
        self.provider.metadata()
    }

    pub fn get_client(&self) -> Client {
        Client::new(String::default(), self.provider.clone())
    }

    pub fn get_named_client(&self, name: String) -> Client {
        Client::new(name, self.provider.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::*;

    #[tokio::test]
    async fn set_provider() {
        let provider = FixedValueProviderBuilder::default()
            .bool_value(true)
            .build()
            .unwrap();

        let api = OpenFeature::new(provider, EvaluationContext::default());

        let client = api.get_client();
        let value = client.get_bool_value("some-key", false, None).await;

        assert_eq!(true, value);
    }
}
