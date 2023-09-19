use std::sync::Arc;

use async_trait::async_trait;
use typed_builder::TypedBuilder;

use crate::{EvaluationContext, StructValue};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

const PROVIDER_NAME: &'static str = "Fixed Value";

// --------------------------------------------------------------------
//  FixedValueProvider
// --------------------------------------------------------------------

#[derive(TypedBuilder, Debug)]
pub struct FixedValueProvider {
    #[builder(default)]
    metadata: ProviderMetadata,

    #[builder(default)]
    bool_value: bool,

    #[builder(default)]
    int_value: i64,

    #[builder(default)]
    float_value: f64,

    #[builder(default)]
    string_value: String,

    #[builder(default)]
    struct_value: Arc<StructValue>,
}

impl Default for FixedValueProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new(PROVIDER_NAME),
            bool_value: Default::default(),
            int_value: Default::default(),
            float_value: Default::default(),
            string_value: Default::default(),
            struct_value: Arc::new(DummyStruct::default().into()),
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
        _evaluation_context: &EvaluationContext,
    ) -> ResolutionDetails<bool> {
        ResolutionDetails::new(self.bool_value)
    }

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        _default_value: i64,
        _evaluation_context: &EvaluationContext,
    ) -> ResolutionDetails<i64> {
        ResolutionDetails::new(self.int_value)
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        _default_value: f64,
        _evaluation_context: &EvaluationContext,
    ) -> ResolutionDetails<f64> {
        ResolutionDetails::new(self.float_value)
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        _default_value: &str,
        _evaluation_context: &EvaluationContext,
    ) -> ResolutionDetails<String> {
        ResolutionDetails::new(self.string_value.clone())
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        _default_value: StructValue,
        _evaluation_context: &EvaluationContext,
    ) -> ResolutionDetails<StructValue> {
        ResolutionDetails::new((*self.struct_value).clone())
    }
}

// --------------------------------------------------------------------
//  DummyStruct
// --------------------------------------------------------------------

#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct DummyStruct {
    #[builder(default)]
    id: i64,

    #[builder(default, setter(into))]
    name: String,
}

impl From<DummyStruct> for StructValue {
    fn from(value: DummyStruct) -> Self {
        StructValue::default()
            .with_field("id", value.id)
            .with_field("name", value.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn from_dummy_struct() {
        let value = DummyStruct::builder().id(100).name("Alex").build();

        let result: StructValue = value.into();

        let expected = StructValue::default()
            .with_field("id", 100)
            .with_field("name", "Alex");

        assert_eq!(expected, result);
    }
}
