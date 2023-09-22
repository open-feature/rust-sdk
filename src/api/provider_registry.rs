use std::sync::Arc;
use std::{borrow::Borrow, collections::HashMap};

use tokio::sync::RwLock;

use crate::provider::{FeatureProvider, NoOpProvider};

use super::global_evaluation_context::GlobalEvaluationContext;

// ============================================================
//  ProviderRegistry
// ============================================================

#[derive(Clone)]
pub struct ProviderRegistry {
    global_evaluation_context: GlobalEvaluationContext,
    providers: Arc<RwLock<HashMap<String, FeatureProviderWrapper>>>,
}

impl ProviderRegistry {
    pub fn new(evaluation_context: GlobalEvaluationContext) -> Self {
        let mut providers: HashMap<String, FeatureProviderWrapper> = HashMap::new();
        providers.insert(
            String::default(),
            FeatureProviderWrapper::new(NoOpProvider::default()),
        );

        Self {
            global_evaluation_context: evaluation_context,
            providers: Arc::new(RwLock::new(providers)),
        }
    }

    pub async fn set_default<T: FeatureProvider>(&self, mut provider: T) {
        let mut map = self.providers.write().await;
        map.remove("");

        provider
            .initialize(&self.global_evaluation_context.get().await.borrow())
            .await;

        map.insert(String::default(), FeatureProviderWrapper::new(provider));
    }

    pub async fn set_named<T: FeatureProvider>(&self, name: &str, mut provider: T) {
        // Drop the already registered provider if any.
        if let Some(_) = self.get_named(name).await {
            self.providers.write().await.remove(name);
        }

        provider
            .initialize(&self.global_evaluation_context.get().await.borrow())
            .await;

        self.providers
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
        self.providers.read().await.get("").unwrap().clone()
    }

    pub async fn get_named(&self, name: &str) -> Option<FeatureProviderWrapper> {
        self.providers
            .read()
            .await
            .get(name)
            .map(|provider| provider.clone())
    }

    pub async fn clear(&self) {
        self.providers.write().await.clear();
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new(GlobalEvaluationContext::default())
    }
}

// ============================================================
//  FeatureProviderWrapper
// ============================================================

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
