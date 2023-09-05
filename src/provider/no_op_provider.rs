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

impl FeatureProvider for NoOpProvider {
    fn metadata(&self) -> &super::ProviderMetadata {
        &self.metadata
    }

    fn resolve_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<crate::EvaluationContext>,
    ) -> ResolutionDetails<bool> {
        ResolutionDetails {
            value: default_value,
            variant: Some("NoOp".to_string()),
            reason: Some(EvaluationReason::Static),
            error: None,
            flag_metadata: Some(FlagMetadata::default()),
        }
    }
}
