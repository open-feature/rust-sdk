use crate::{EvaluationReason, FlagMetadata};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

/// The default provider that simply returns given default value during evaluation.
#[derive(Default, Debug)]
pub struct NoOpProvider {
    metadata: ProviderMetadata,
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
