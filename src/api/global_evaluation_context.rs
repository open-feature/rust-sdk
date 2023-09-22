use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::EvaluationContext;

#[derive(Clone, Default)]
pub struct GlobalEvaluationContext(Arc<RwLock<EvaluationContext>>);

impl GlobalEvaluationContext {
    pub fn new(evaluation_context: EvaluationContext) -> Self {
        Self(Arc::new(RwLock::new(evaluation_context)))
    }

    pub async fn get(&self) -> RwLockReadGuard<EvaluationContext> {
        self.0.read().await
    }

    pub async fn get_mut(&self) -> RwLockWriteGuard<EvaluationContext> {
        self.0.write().await
    }
}
