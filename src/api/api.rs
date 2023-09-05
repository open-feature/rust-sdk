use std::sync::Arc;

use lazy_static::lazy_static;

use crate::{
    provider::{FeatureProvider, NoOpProvider, ProviderMetadata},
    Client,
};

use super::client::ClientMetadata;

lazy_static! {
    /// The singleton instance of [`OpenFeature`] struct.
    /// The client should always use this instance to access OpenFeature APIs.
    pub static ref SINGLETON: OpenFeature = OpenFeature {
        provider: Arc::new(NoOpProvider::default())
    };
}

/// THE struct of the OpenFeature API.
/// Access it via the [`SINGLETON`] instance.
pub struct OpenFeature {
    provider: Arc<dyn FeatureProvider + Send + Sync>,
}

impl OpenFeature {
    pub fn set_provider<T>(mut self, provider: T)
    where
        T: FeatureProvider + Send + Sync + 'static,
    {
        self.provider = Arc::new(provider);
    }

    pub fn provider_metadata(&self) -> &ProviderMetadata {
        self.provider.metadata()
    }

    pub fn get_client(&self, name: String) -> Client {
        Client::new(name, self.provider.clone())
    }
}
