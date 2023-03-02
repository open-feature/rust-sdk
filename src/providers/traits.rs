use crate::evaluation::FlattenedContext;

use super::types::{ProviderMetadata, ResolutionDetails};

pub trait FeatureProvider {
    fn new() -> Self;
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
