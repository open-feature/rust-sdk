use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::HookWrapper;

#[derive(Clone, Default)]
pub struct GlobalHooks(Arc<RwLock<Vec<HookWrapper>>>);

impl GlobalHooks {
    pub async fn get(&self) -> RwLockReadGuard<'_, Vec<HookWrapper>> {
        self.0.read().await
    }

    pub async fn get_mut(&self) -> RwLockWriteGuard<'_, Vec<HookWrapper>> {
        self.0.write().await
    }
}
