use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
    pub fn singleton() -> RwLockReadGuard<'static, Self> {
        SINGLETON.read().unwrap()
    }

    pub fn singleton_mut() -> RwLockWriteGuard<'static, Self> {
        SINGLETON.write().unwrap()
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
    use std::thread;

    use super::*;
    use crate::provider::*;

    impl OpenFeature {
        pub fn new<T: FeatureProvider>(provider: T, evaluation_context: EvaluationContext) -> Self {
            Self {
                provider: Arc::new(provider),
                evaluation_context,
            }
        }
    }

    #[tokio::test]
    async fn set_provider() {
        let provider = FixedValueProvider::builder().bool_value(true).build();

        let api = OpenFeature::new(provider, EvaluationContext::default());

        let client = api.get_client();
        let value = client.get_bool_value("some-key", false, None).await;

        assert_eq!(true, value);
    }

    #[tokio::test]
    async fn test_singleton_multi_thread() {
        let reader1 = thread::spawn(|| {
            let _ = OpenFeature::singleton().provider_metadata();
        });

        let writer = thread::spawn(|| {
            OpenFeature::singleton_mut().set_provider(FixedValueProvider::default());
        });

        let reader2 = thread::spawn(|| {
            let _ = OpenFeature::singleton().provider_metadata();
        });

        let _ = (reader1.join(), reader2.join(), writer.join());

        assert_eq!(
            "Fixed Value",
            OpenFeature::singleton().provider_metadata().name
        );
    }
}
