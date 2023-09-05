use async_trait::async_trait;

use crate::{EvaluationReason, FlagMetadata};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

const PROVIDER_NAME: &'static str = "No Operation";

/// The default provider that simply returns given default value during evaluation.
#[derive(Debug)]
pub struct NoOpProvider {
    metadata: ProviderMetadata,
}

impl NoOpProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME.to_string()),
        }
    }
}

impl Default for NoOpProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME.to_string()),
        }
    }
}

#[async_trait]
impl FeatureProvider for NoOpProvider {
    fn metadata(&self) -> &super::ProviderMetadata {
        &self.metadata
    }

    async fn resolve_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<crate::EvaluationContext>,
    ) -> ResolutionDetails<bool> {
        ResolutionDetails::new(default_value)
    }
}
