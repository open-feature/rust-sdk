use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    provider::{FeatureProvider, NoOpProvider},
    EvaluationContext,
};

// ====================================================================
//  ProviderRegistry
// ====================================================================

#[derive(Clone)]
pub struct ProviderRegistry(Arc<RwLock<HashMap<String, FeatureProviderWrapper>>>);

impl ProviderRegistry {
    pub async fn set_default<T: FeatureProvider>(&self, mut provider: T) {
        let mut map = self.0.write().await;
        map.remove("");

        provider.initialize(EvaluationContext::default()).await;

        map.insert(String::default(), FeatureProviderWrapper::new(provider));
    }

    pub async fn set_named<T: FeatureProvider>(&self, name: &str, mut provider: T) {
        // Drop the already registered provider if any.
        if let Some(_) = self.get_named(name).await {
            self.0.write().await.remove(name);
        }

        provider.initialize(EvaluationContext::default()).await;

        self.0
            .write()
            .await
            .insert(name.to_string(), FeatureProviderWrapper::new(provider));
    }

    pub async fn get(&self, name: &str) -> FeatureProviderWrapper {
        match self.get_named(name).await {
            Some(provider) => provider,
            None => self.get_default().await,
        }
    }

    pub async fn get_default(&self) -> FeatureProviderWrapper {
        self.0.read().await.get("").unwrap().clone()
    }

    pub async fn get_named(&self, name: &str) -> Option<FeatureProviderWrapper> {
        self.0
            .read()
            .await
            .get(name)
            .map(|provider| provider.clone())
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        let mut providers: HashMap<String, FeatureProviderWrapper> = HashMap::new();
        providers.insert(
            String::default(),
            FeatureProviderWrapper::new(NoOpProvider::new()),
        );

        Self(Arc::new(RwLock::new(providers)))
    }
}

#[derive(Clone)]
pub struct FeatureProviderWrapper(Arc<dyn FeatureProvider>);

impl FeatureProviderWrapper {
    pub fn new(provider: impl FeatureProvider) -> Self {
        Self(Arc::new(provider))
    }

    pub fn get(&self) -> Arc<dyn FeatureProvider> {
        self.0.clone()
    }
}
