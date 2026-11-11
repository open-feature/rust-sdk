use async_trait::async_trait;

use crate::{
    EvaluationContext, EvaluationError, EvaluationErrorCode, EvaluationResult, StructValue,
};

use super::{FeatureProvider, ProviderMetadata, ProviderStatus, ResolutionDetails};

// ============================================================
//  NoOpProvider
// ============================================================

/// The default provider that does nothing.
///
/// It always returns [`EvaluationError`] for all the given flag keys.
#[derive(Debug)]
pub struct NoOpProvider {
    metadata: ProviderMetadata,
}

impl Default for NoOpProvider {
    fn default() -> Self {
        Self {
            metadata: ProviderMetadata::new("No-op Provider"),
        }
    }
}

#[async_trait]
impl FeatureProvider for NoOpProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    fn status(&self) -> ProviderStatus {
        ProviderStatus::NotReady
    }

    async fn resolve_bool_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<bool>> {
        just_error()
    }

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<i64>> {
        just_error()
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<f64>> {
        just_error()
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<String>> {
        just_error()
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<StructValue>, EvaluationError> {
        just_error()
    }
}

fn just_error<T>() -> EvaluationResult<T> {
    Err(EvaluationError::builder()
        .code(EvaluationErrorCode::ProviderNotReady)
        .message("No-op provider is never ready")
        .build())
}

// ============================================================
//  Tests
// ============================================================

#[cfg(test)]
mod tests {
    use spec::spec;

    use super::*;
    use crate::{provider::ProviderStatus, *};

    #[spec(
        number = "2.1.1",
        text = "The provider interface MUST define a metadata member or accessor, containing a name field or accessor of type string, which identifies the provider implementation."
    )]
    #[test]
    fn metadata_name() {
        let provider = NoOpProvider::default();

        assert_eq!(provider.metadata().name, "No-op Provider");
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
        let provider = NoOpProvider::default();
        let context = EvaluationContext::default();

        assert!(provider.resolve_bool_value("", &context).await.is_err());
        assert!(provider.resolve_int_value("", &context).await.is_err());
        assert!(provider.resolve_float_value("", &context).await.is_err());
        assert!(provider.resolve_string_value("", &context).await.is_err());
        assert!(provider.resolve_struct_value("", &context).await.is_err());
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
        assert_eq!(provider.status(), ProviderStatus::NotReady);
    }

    #[spec(
        number = "2.5.1",
        text = "The provider MAY define a mechanism to gracefully shutdown and dispose of resources."
    )]
    #[tokio::test]
    async fn shutdown() {
        let provider = NoOpProvider::default();
        // NoOpProvider has a default empty shutdown implementation
        provider.shutdown().await;
    }
}
