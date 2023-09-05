use async_trait::async_trait;

use crate::{EvaluationContext, EvaluationReason, FlagMetadata};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

const PROVIDER_NAME: &'static str = "Fixed Value";

#[derive(Debug)]
pub struct FixedValueProvider {
    metadata: ProviderMetadata,
    bool_value: bool,
}

impl FixedValueProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME.to_string()),
            bool_value: bool::default(),
        }
    }

    pub fn with_bool_value(mut self, bool_value: bool) -> Self {
        self.bool_value = bool_value;
        self
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

#[async_trait]
impl FeatureProvider for FixedValueProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn resolve_bool_value(
        &self,
        _flag_key: &str,
        _default_value: bool,
        _evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<bool> {
        ResolutionDetails::new(self.bool_value)
    }
}
