use std::sync::Arc;

use async_trait::async_trait;
use typed_builder::TypedBuilder;

use crate::{
    EvaluationContext, EvaluationContextFieldValue, EvaluationError, EvaluationReason,
    FlagMetadata, StructValue,
};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

const PROVIDER_NAME: &'static str = "No Operation";

// ============================================================
//  NoOpProvider
// ============================================================

/// The default provider that does nothing.
///
/// By default, it returns the default value of each supported type. You can inject values (for
/// testing purpose or simply providing some fixed value) by the following ways:
/// 1. When creating an instance, inject values through the builder.
/// 2. Supply a value with key "Value" in the evaluation context. Because the evaluation context is
///    merged, you can achieve this by providing value in API level, client level or invocation
///    level. Note that the type must be compliant for it to work.
///
/// Other tips:
/// * It will return reason `Default` when the value equals to the default, `Static` otherwise.
/// * It will return variant `"Default"` or `"Static"` respectively.
/// * It will return flag metadata with key `"Type"` and a string value that corresponds to the
///   real type, EXCEPT for `resolve_struct_value`.
#[derive(TypedBuilder, Debug)]
pub struct NoOpProvider {
    #[builder(default)]
    metadata: ProviderMetadata,

    #[builder(default)]
    bool_value: bool,

    #[builder(default)]
    int_value: i64,

    #[builder(default)]
    float_value: f64,

    #[builder(default, setter(into))]
    string_value: String,

    #[builder(default, setter(into))]
    struct_value: Arc<StructValue>,
}

impl NoOpProvider {
    fn create_reason_variant(is_default: bool) -> (EvaluationReason, String) {
        if is_default {
            (EvaluationReason::Default, "Default".to_string())
        } else {
            (EvaluationReason::Static, "Static".to_string())
        }
    }
}

impl Default for NoOpProvider {
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
impl FeatureProvider for NoOpProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn resolve_bool_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<bool>, EvaluationError> {
        let value = match evaluation_context.custom_fields.get("Value") {
            Some(v) => match *v {
                EvaluationContextFieldValue::Bool(value) => value,
                _ => self.bool_value,
            },
            None => self.bool_value,
        };

        let (reason, variant) = Self::create_reason_variant(value == Default::default());

        Ok(ResolutionDetails::builder()
            .value(value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(FlagMetadata::default().with_value("Type", "Bool"))
            .build())
    }

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<i64>, EvaluationError> {
        let value = match evaluation_context.custom_fields.get("Value") {
            Some(v) => match *v {
                EvaluationContextFieldValue::Int(value) => value,
                _ => self.int_value,
            },
            None => self.int_value,
        };

        let (reason, variant) = Self::create_reason_variant(value == Default::default());

        Ok(ResolutionDetails::builder()
            .value(value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(FlagMetadata::default().with_value("Type", "Int"))
            .build())
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<f64>, EvaluationError> {
        let value = match evaluation_context.custom_fields.get("Value") {
            Some(v) => match *v {
                EvaluationContextFieldValue::Float(value) => value,
                _ => self.float_value,
            },
            None => self.float_value,
        };

        let (reason, variant) = Self::create_reason_variant(value == Default::default());

        Ok(ResolutionDetails::builder()
            .value(value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(FlagMetadata::default().with_value("Type", "Float"))
            .build())
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<String>, EvaluationError> {
        let value = match evaluation_context.custom_fields.get("Value") {
            Some(v) => match v {
                EvaluationContextFieldValue::String(value) => value.to_string(),
                _ => self.string_value.clone(),
            },

            None => self.string_value.clone(),
        };

        let (reason, variant) = Self::create_reason_variant(value == String::default());

        Ok(ResolutionDetails::builder()
            .value(value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(FlagMetadata::default().with_value("Type", "String"))
            .build())
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<StructValue>, EvaluationError> {
        let (reason, variant) =
            Self::create_reason_variant(self.struct_value == Default::default());

        Ok(ResolutionDetails::builder()
            .value((*self.struct_value).clone())
            .reason(reason)
            .variant(variant)
            .build())
    }
}

// ============================================================
//  DummyStruct
// ============================================================

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
