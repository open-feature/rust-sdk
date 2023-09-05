use crate::{EvaluationContext, EvaluationReason, FlagMetadata};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

const PROVIDER_NAME: &'static str = "Fixed Value";

#[derive(Debug)]
pub struct FixedValueProvider {
    metadata: ProviderMetadata,
    bool_value: bool,
}

impl FixedValueProvider {
    pub fn new(bool_value: bool) -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME.to_string()),
            bool_value,
        }
    }
}

impl Default for FixedValueProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME.to_string()),
            bool_value: Default::default(),
        }
    }
}

impl FeatureProvider for FixedValueProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    fn resolve_bool_value(
        &self,
        _flag_key: &str,
        _default_value: bool,
        _evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<bool> {
        ResolutionDetails::new_successful(
            self.bool_value,
            "".to_string(),
            EvaluationReason::Static,
            FlagMetadata::default(),
        )
    }
}
