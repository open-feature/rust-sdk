use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::{EvaluationContext, EvaluationReason, FlagMetadata};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

const PROVIDER_NAME: &'static str = "Fixed Value";

#[derive(Debug)]
pub struct FixedValueProvider {
    metadata: ProviderMetadata,
    bool_value: bool,
    int_value: i64,
    float_value: f64,
    string_value: String,
    struct_value: Arc<dyn Any + Send + Sync>,
}

impl FixedValueProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_bool_value(mut self, value: bool) -> Self {
        self.bool_value = value;
        self
    }

    pub fn with_int_value(mut self, vaule: i64) -> Self {
        self.int_value = vaule;
        self
    }

    pub fn with_float_value(mut self, value: f64) -> Self {
        self.float_value = value;
        self
    }

    pub fn with_string_value(mut self, value: String) -> Self {
        self.string_value = value;
        self
    }

    pub fn with_struct_value<T>(mut self, value: T) -> Self
    where
        T: Any + Send + Sync,
    {
        self.struct_value = Arc::new(value);
        self
    }
}

impl Default for FixedValueProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME.to_string()),
            bool_value: Default::default(),
            int_value: Default::default(),
            float_value: Default::default(),
            string_value: Default::default(),
            struct_value: Arc::new(String::default()),
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

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        _default_value: i64,
        _evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<i64> {
        ResolutionDetails::new(self.int_value)
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        _default_value: f64,
        _evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<f64> {
        ResolutionDetails::new(self.float_value)
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        _default_value: &str,
        _evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<String> {
        ResolutionDetails::new(self.string_value.clone())
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        _default_value: Arc<dyn Any + Send + Sync>,
        _evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<Arc<dyn Any + Send + Sync>> {
        ResolutionDetails::new(self.struct_value.clone())
    }
}
