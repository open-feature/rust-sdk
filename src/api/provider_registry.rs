use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{borrow::Borrow, collections::HashMap};

use tokio::sync::RwLock;
use tokio::task::JoinHandle;

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
        let old_provider = self.providers.write().await.remove("");

        if let Some(old_provider) = old_provider {
            old_provider.shutdown_in_background();
        }

        provider
            .initialize(self.global_evaluation_context.get().await.borrow())
            .await;

        self.providers
            .write()
            .await
            .insert(String::default(), FeatureProviderWrapper::new(provider));
    }

    pub async fn set_named<T: FeatureProvider>(&self, name: &str, mut provider: T) {
        // Drop the already registered provider if any.
        if let Some(old_provider) = self.providers.write().await.remove(name) {
            old_provider.shutdown_in_background();
        }

        provider
            .initialize(self.global_evaluation_context.get().await.borrow())
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
        self.providers.read().await.get(name).cloned()
    }

    pub async fn clear(&self) {
        let providers: Vec<FeatureProviderWrapper> =
            self.providers.read().await.values().cloned().collect();

        let mut shutdown_handles = Vec::with_capacity(providers.len());
        for provider in providers {
            if let Some(handle) = provider.shutdown_in_background() {
                shutdown_handles.push(handle);
            }
        }

        self.providers.write().await.clear();

        for handle in shutdown_handles {
            let _ = handle.await;
        }
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
pub struct FeatureProviderWrapper(Arc<ProviderEntry>);

impl FeatureProviderWrapper {
    pub fn new(provider: impl FeatureProvider) -> Self {
        Self(Arc::new(ProviderEntry::new(provider)))
    }

    pub fn get(&self) -> Arc<dyn FeatureProvider> {
        self.0.provider.clone()
    }

    pub fn shutdown_in_background(&self) -> Option<JoinHandle<()>> {
        if self
            .0
            .shutdown_started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let provider = self.get();
            Some(tokio::spawn(async move {
                provider.shutdown().await;
            }))
        } else {
            None
        }
    }
}

struct ProviderEntry {
    provider: Arc<dyn FeatureProvider>,
    shutdown_started: AtomicBool,
}

impl ProviderEntry {
    fn new(provider: impl FeatureProvider) -> Self {
        Self {
            provider: Arc::new(provider),
            shutdown_started: AtomicBool::new(false),
        }
    }
}
