use async_trait::async_trait;

use crate::evaluation::FlattenedContext;

use super::types::{Configuration, ProviderMetadata, ResolutionDetails};

#[async_trait]
pub trait FeatureProvider<P> {
    fn new(conf: Configuration) -> Self;
    async fn connect(&self);
    fn meta_data(&self) -> ProviderMetadata;
    fn resolution<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: FlattenedContext,
    ) -> anyhow::Result<ResolutionDetails<T>>
    where
        T: Clone;
}
