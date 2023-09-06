use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::{EvaluationContext, StructValue};

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
            metadata: ProviderMetadata::new(PROVIDER_NAME),
        }
    }
}

impl Default for NoOpProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME),
        }
    }
}

#[async_trait]
impl FeatureProvider for NoOpProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn resolve_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<bool> {
        ResolutionDetails::new(default_value)
    }

    async fn resolve_int_value(
        &self,
        flag_key: &str,
        default_value: i64,
        evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<i64> {
        ResolutionDetails::new(default_value)
    }

    async fn resolve_float_value(
        &self,
        flag_key: &str,
        default_value: f64,
        evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<f64> {
        ResolutionDetails::new(default_value)
    }

    async fn resolve_string_value(
        &self,
        flag_key: &str,
        default_value: &str,
        evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<String> {
        ResolutionDetails::new(default_value.to_string())
    }

    async fn resolve_struct_value(
        &self,
        flag_key: &str,
        default_value: StructValue,
        evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<StructValue> {
        ResolutionDetails::new(default_value)
    }
}
