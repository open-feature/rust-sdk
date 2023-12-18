use std::sync::Arc;

use async_trait::async_trait;
use typed_builder::TypedBuilder;

use crate::{
    EvaluationContext, EvaluationContextFieldValue, EvaluationError, EvaluationReason,
    FlagMetadata, FlagMetadataValue, StructValue,
};

use super::{FeatureProvider, ProviderMetadata, ResolutionDetails};

// ============================================================
//  NoOpProvider
// ============================================================

/// The default provider that does nothing.
///
/// By default, it returns the default value of each supported type. You can inject values (for
/// testing purpose or simply providing some fixed value) when creating an instance.
///
/// Other tips:
/// * It will return reason `Default` when the value equals to the default, `Static` otherwise.
/// * It will return variant `"Default"` or `"Static"` respectively.
/// * It will return flag metadata with key `"Type"` and a string value that corresponds to the
///   real type, EXCEPT for `resolve_struct_value`.
/// * It will return flag metadata with keys "TargetingKey" and value of targeting key extracted
/// from the evaluation context, if the value is not `None`.
/// * It will return flag metadata with keys/values extracted from the evaluation context, as long
/// as the value is a bool, number or string.
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

    fn populate_evaluation_context_values(
        flag_metadata: &mut FlagMetadata,
        evaluation_context: &EvaluationContext,
    ) {
        if let Some(value) = &evaluation_context.targeting_key {
            flag_metadata.add_value("TargetingKey", value.clone());
        }

        evaluation_context
            .custom_fields
            .iter()
            .for_each(|(key, value)| match value {
                EvaluationContextFieldValue::Bool(value) => {
                    flag_metadata.add_value(key, FlagMetadataValue::Bool(*value));
                }
                EvaluationContextFieldValue::Int(value) => {
                    flag_metadata.add_value(key, FlagMetadataValue::Int(*value));
                }
                EvaluationContextFieldValue::Float(value) => {
                    flag_metadata.add_value(key, FlagMetadataValue::Float(*value));
                }
                EvaluationContextFieldValue::String(value) => {
                    flag_metadata.add_value(key, FlagMetadataValue::String(value.clone()));
                }
                _ => (),
            });
    }
}

impl Default for NoOpProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new("No Operation - Default"),
            bool_value: Default::default(),
            int_value: Default::default(),
            float_value: Default::default(),
            string_value: String::default(),
            struct_value: Arc::new(DummyStruct::default().into()),
        }
    }
}

#[async_trait]
impl FeatureProvider for NoOpProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn initialize(&mut self, _evaluation_context: &EvaluationContext) {
        self.metadata = ProviderMetadata::new("No Operation");
    }

    async fn resolve_bool_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<bool>, EvaluationError> {
        let (reason, variant) = Self::create_reason_variant(self.bool_value == bool::default());

        let mut flag_metadata = FlagMetadata::default().with_value("Type", "Bool");
        Self::populate_evaluation_context_values(&mut flag_metadata, evaluation_context);

        Ok(ResolutionDetails::builder()
            .value(self.bool_value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(flag_metadata)
            .build())
    }

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<i64>, EvaluationError> {
        let (reason, variant) = Self::create_reason_variant(self.int_value == i64::default());

        let mut flag_metadata = FlagMetadata::default().with_value("Type", "Int");
        Self::populate_evaluation_context_values(&mut flag_metadata, evaluation_context);

        Ok(ResolutionDetails::builder()
            .value(self.int_value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(flag_metadata)
            .build())
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<f64>, EvaluationError> {
        let (reason, variant) =
            Self::create_reason_variant((self.float_value - f64::default()).abs() < f64::EPSILON);

        let mut flag_metadata = FlagMetadata::default().with_value("Type", "Float");
        Self::populate_evaluation_context_values(&mut flag_metadata, evaluation_context);

        Ok(ResolutionDetails::builder()
            .value(self.float_value)
            .reason(reason)
            .variant(variant)
            .flag_metadata(flag_metadata)
            .build())
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<String>, EvaluationError> {
        let (reason, variant) = Self::create_reason_variant(self.string_value == String::default());

        let mut flag_metadata = FlagMetadata::default().with_value("Type", "String");
        Self::populate_evaluation_context_values(&mut flag_metadata, evaluation_context);

        Ok(ResolutionDetails::builder()
            .value(self.string_value.clone())
            .reason(reason)
            .variant(variant)
            .flag_metadata(flag_metadata)
            .build())
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<StructValue>, EvaluationError> {
        let (reason, variant) = Self::create_reason_variant(self.struct_value == Arc::default());

        let mut flag_metadata = FlagMetadata::default().with_value("Type", "Struct");
        Self::populate_evaluation_context_values(&mut flag_metadata, evaluation_context);

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

// ============================================================
//  Tests
// ============================================================

#[cfg(test)]
mod tests {
    use spec::spec;

    use super::*;
    use crate::{provider::ProviderStatus, *};

    #[test]
    fn from_dummy_struct() {
        let value = DummyStruct::builder().id(100).name("Alex").build();

        let result: StructValue = value.into();

        let expected = StructValue::default()
            .with_field("id", 100)
            .with_field("name", "Alex");

        assert_eq!(expected, result);
    }

    #[spec(
        number = "2.1.1",
        text = "The provider interface MUST define a metadata member or accessor, containing a name field or accessor of type string, which identifies the provider implementation."
    )]
    #[test]
    fn metadata_name() {
        let provider = NoOpProvider::default();

        assert_eq!(provider.metadata().name, "No Operation - Default");
    }

    #[spec(
        number = "2.2.1",
        text = "The feature provider interface MUST define methods to resolve flag values, with parameters flag key (string, required), default value (boolean | number | string | structure, required) and evaluation context (optional), which returns a resolution details structure."
    )]
    #[spec(
        number = "2.2.2.1",
        text = "The feature provider interface MUST define methods for typed flag resolution, including boolean, numeric, string, and structure."
    )]
    #[spec(
        number = "2.2.3",
        text = "In cases of normal execution, the provider MUST populate the resolution details structure's value field with the resolved flag value."
    )]
    #[spec(
        number = "2.2.4",
        text = "In cases of normal execution, the provider SHOULD populate the resolution details structure's variant field with a string identifier corresponding to the returned flag value."
    )]
    #[spec(
        number = "2.2.5",
        text = r###"The provider SHOULD populate the resolution details structure's reason field with "STATIC", "DEFAULT", "TARGETING_MATCH", "SPLIT", "CACHED", "DISABLED", "UNKNOWN", "STALE", "ERROR" or some other string indicating the semantic reason for the returned flag value."###
    )]
    #[spec(
        number = "2.2.6",
        text = "In cases of normal execution, the provider MUST NOT populate the resolution details structure's error code field, or otherwise must populate it with a null or falsy value."
    )]
    #[spec(
        number = "2.2.9",
        text = "The provider SHOULD populate the resolution details structure's flag metadata field. "
    )]
    #[spec(
        number = "2.2.10",
        text = "flag metadata MUST be a structure supporting the definition of arbitrary properties, with keys of type string, and values of type boolean | string | number."
    )]
    #[tokio::test]
    async fn resolve_value() {
        let provider = NoOpProvider::builder()
            .bool_value(true)
            .int_value(100)
            .string_value("Hello")
            .struct_value(StructValue::default().with_field("Key", "Value"))
            .build();

        // Check bool.
        let result = provider
            .resolve_bool_value("key", &EvaluationContext::default())
            .await
            .unwrap();

        assert_eq!(result.value, true);
        assert_eq!(result.reason, Some(EvaluationReason::Static));
        assert_eq!(result.variant, Some("Static".to_string()));
        assert_eq!(
            result.flag_metadata,
            Some(FlagMetadata::default().with_value("Type", "Bool"))
        );

        // Check int.
        let result = provider
            .resolve_int_value("key", &EvaluationContext::default())
            .await
            .unwrap();

        assert_eq!(result.value, 100);
        assert_eq!(result.reason, Some(EvaluationReason::Static));
        assert_eq!(result.variant, Some("Static".to_string()));
        assert_eq!(
            result.flag_metadata,
            Some(FlagMetadata::default().with_value("Type", "Int"))
        );

        // Check float.
        let result = provider
            .resolve_float_value("key", &EvaluationContext::default())
            .await
            .unwrap();

        assert_eq!(result.value, 0.0);
        assert_eq!(result.reason, Some(EvaluationReason::Default));
        assert_eq!(result.variant, Some("Default".to_string()));
        assert_eq!(
            result.flag_metadata,
            Some(FlagMetadata::default().with_value("Type", "Float"))
        );

        // Check string.
        let result = provider
            .resolve_string_value("key", &EvaluationContext::default())
            .await
            .unwrap();

        assert_eq!(result.value, "Hello");
        assert_eq!(result.reason, Some(EvaluationReason::Static));
        assert_eq!(result.variant, Some("Static".to_string()));
        assert_eq!(
            result.flag_metadata,
            Some(FlagMetadata::default().with_value("Type", "String"))
        );

        // Check struct.
        let result = provider
            .resolve_struct_value("key", &EvaluationContext::default())
            .await
            .unwrap();

        assert_eq!(
            result.value,
            StructValue::default().with_field("Key", "Value")
        );
        assert_eq!(result.reason, Some(EvaluationReason::Static));
        assert_eq!(result.variant, Some("Static".to_string()));
        assert_eq!(result.flag_metadata, None);
    }

    #[spec(
        number = "2.2.7",
        text = "In cases of abnormal execution, the provider MUST indicate an error using the idioms of the implementation language, with an associated error code and optional associated error message."
    )]
    #[test]
    fn error_code_message_provided_checked_by_type_system() {}

    #[spec(
        number = "2.2.8.1",
        text = "The resolution details structure SHOULD accept a generic argument (or use an equivalent language feature) which indicates the type of the wrapped value field."
    )]
    #[test]
    fn resolution_details_generic_checked_by_type_system() {}

    #[spec(
        number = "2.4.1",
        text = "The provider MAY define an initialize function which accepts the global evaluation context as an argument and performs initialization logic relevant to the provider."
    )]
    #[tokio::test]
    async fn initialize() {
        let mut provider = NoOpProvider::default();

        provider.initialize(&EvaluationContext::default()).await;
    }

    #[spec(
        number = "2.4.2",
        text = "The provider MAY define a status field/accessor which indicates the readiness of the provider, with possible values NOT_READY, READY, or ERROR."
    )]
    #[spec(
        number = "2.4.3",
        text = "The provider MUST set its status field/accessor to READY if its initialize function terminates normally."
    )]
    #[spec(
        number = "2.4.4",
        text = "The provider MUST set its status field to ERROR if its initialize function terminates abnormally."
    )]
    #[spec(
        number = "2.4.5",
        text = "The provider SHOULD indicate an error if flag resolution is attempted before the provider is ready."
    )]
    #[tokio::test]
    async fn status() {
        let provider = NoOpProvider::default();
        assert_eq!(provider.status(), ProviderStatus::Ready);
    }

    #[spec(
        number = "2.5.1",
        text = "The provider MAY define a mechanism to gracefully shutdown and dispose of resources."
    )]
    #[test]
    fn shutdown_covered_by_drop_trait() {}
}
