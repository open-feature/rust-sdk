use std::sync::Arc;

use crate::provider::FeatureProvider;

/// The metadata of OpenFeature client.
pub struct ClientMetadata {
    name: String,
}

/// The OpenFeature client.
/// Create it through the [`OpenFeature`] struct.
pub struct Client {
    pub metadata: ClientMetadata,
    provider: Arc<dyn FeatureProvider + Send + Sync + 'static>,
}

impl Client {
    pub fn new(name: String, provider: Arc<dyn FeatureProvider + Send + Sync + 'static>) -> Self {
        Self {
            metadata: ClientMetadata { name },
            provider,
        }
    }
}
