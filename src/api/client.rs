use std::sync::Arc;

use crate::{
    provider::{FeatureProvider, ResolutionDetails},
    EvaluationContext, EvaluationDetails, EvaluationError, EvaluationErrorCode, EvaluationOptions,
    EvaluationResult, StructValue,
};

use super::{
    global_evaluation_context::GlobalEvaluationContext, provider_registry::ProviderRegistry,
};

/// The metadata of OpenFeature client.
pub struct ClientMetadata {
    /// The name of client.
    pub name: String,
}

/// The OpenFeature client.
/// Create it through the [`OpenFeature`] struct.
pub struct Client {
    metadata: ClientMetadata,
    provider_registry: ProviderRegistry,
    evaluation_context: EvaluationContext,
    global_evaluation_context: GlobalEvaluationContext,
}

impl Client {
    /// Create a new [`Client`] instance.
    pub fn new(
        name: impl Into<String>,
        global_evaluation_context: GlobalEvaluationContext,
        provider_registry: ProviderRegistry,
    ) -> Self {
        Self {
            metadata: ClientMetadata { name: name.into() },
            global_evaluation_context,
            provider_registry,
            evaluation_context: EvaluationContext::default(),
        }
    }

    /// Return the metadata of current client.
    pub fn metadata(&self) -> &ClientMetadata {
        &self.metadata
    }

    /// Set evaluation context to the client.
    pub fn set_evaluation_context(&mut self, evaluation_context: EvaluationContext) {
        self.evaluation_context = evaluation_context;
    }

    /// Evaluate given `flag_key` with corresponding `evaluation_context` and `evaluation_options`
    /// as a bool value.
    #[allow(unused_variables)]
    pub async fn get_bool_value(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<bool> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_bool_value(flag_key, &context)
            .await?
            .value)
    }

    /// Evaluate given `flag_key` with corresponding `evaluation_context` and `evaluation_options`
    /// as an int (i64) value.
    #[allow(unused_variables)]
    pub async fn get_int_value(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<i64> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_int_value(flag_key, &context)
            .await?
            .value)
    }

    /// Evaluate given `flag_key` with corresponding `evaluation_context` and `evaluation_options`
    /// as a float (f64) value.
    /// If the resolution fails, the `default_value` is returned.
    #[allow(unused_variables)]
    pub async fn get_float_value(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<f64> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_float_value(flag_key, &context)
            .await?
            .value)
    }

    /// Evaluate given `flag_key` with corresponding `evaluation_context` and `evaluation_options`
    /// as a string value.
    /// If the resolution fails, the `default_value` is returned.
    #[allow(unused_variables)]
    pub async fn get_string_value(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<String> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_string_value(flag_key, &context)
            .await?
            .value)
    }

    /// Evaluate given `flag_key` with corresponding `evaluation_context` and `evaluation_options`
    /// as a struct.
    /// If the resolution fails, the `default_value` is returned.
    /// The required type should implement [`From<StructValue>`] trait.
    #[allow(unused_variables)]
    pub async fn get_struct_value<T: TryFrom<StructValue>>(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<T> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        let result = self
            .get_provider()
            .await
            .resolve_struct_value(flag_key, &context)
            .await?;

        match T::try_from(result.value) {
            Ok(t) => Ok(t),
            Err(error) => Err(EvaluationError {
                code: EvaluationErrorCode::TypeMismatch,
                message: Some("Unable to cast value to required type".to_string()),
            }),
        }
    }

    /// Return the [`EvaluationDetails`] with given `flag_key`, `evaluation_context` and
    /// `evaluation_options`.
    #[allow(unused_variables)]
    pub async fn get_bool_details(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<EvaluationDetails<bool>> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_bool_value(flag_key, &context)
            .await?
            .into_evaluation_details(flag_key))
    }

    /// Return the [`EvaluationDetails`] with given `flag_key`, `evaluation_context` and
    /// `evaluation_options`.
    #[allow(unused_variables)]
    pub async fn get_int_details(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<EvaluationDetails<i64>> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_int_value(flag_key, &context)
            .await?
            .into_evaluation_details(flag_key))
    }

    /// Return the [`EvaluationDetails`] with given `flag_key`, `evaluation_context` and
    /// `evaluation_options`.
    #[allow(unused_variables)]
    pub async fn get_float_details(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<EvaluationDetails<f64>> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_float_value(flag_key, &context)
            .await?
            .into_evaluation_details(flag_key))
    }

    /// Return the [`EvaluationDetails`] with given `flag_key`, `evaluation_context` and
    /// `evaluation_options`.
    #[allow(unused_variables)]
    pub async fn get_string_details(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<EvaluationDetails<String>> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        Ok(self
            .get_provider()
            .await
            .resolve_string_value(flag_key, &context)
            .await?
            .into_evaluation_details(flag_key))
    }

    /// Return the [`EvaluationDetails`] with given `flag_key`, `evaluation_context` and
    /// `evaluation_options`.
    #[allow(unused_variables)]
    pub async fn get_struct_details<T: TryFrom<StructValue>>(
        &self,
        flag_key: &str,
        evaluation_context: Option<&EvaluationContext>,
        evaluation_options: Option<&EvaluationOptions>,
    ) -> EvaluationResult<EvaluationDetails<T>> {
        let context = self.merge_evaluation_context(evaluation_context).await;

        let result = self
            .get_provider()
            .await
            .resolve_struct_value(flag_key, &context)
            .await?;

        match T::try_from(result.value) {
            Ok(value) => Ok(EvaluationDetails {
                flag_key: flag_key.to_string(),
                value,
                reason: result.reason,
                variant: result.variant,
                flag_metadata: result.flag_metadata.unwrap_or_default(),
            }),
            Err(error) => Err(EvaluationError {
                code: EvaluationErrorCode::TypeMismatch,
                message: Some("Unable to cast value to required type".to_string()),
            }),
        }
    }

    async fn get_provider(&self) -> Arc<dyn FeatureProvider> {
        self.provider_registry.get(&self.metadata.name).await.get()
    }

    /// Merge provided `flag_evaluation_context` (that is passed when evaluating a flag) with
    /// client and global evaluation context.
    async fn merge_evaluation_context(
        &self,
        flag_evaluation_context: Option<&EvaluationContext>,
    ) -> EvaluationContext {
        let mut context = match flag_evaluation_context {
            Some(c) => c.clone(),
            None => EvaluationContext::default(),
        };

        context.merge_missing(&self.evaluation_context);

        let global_evaluation_context = self.global_evaluation_context.get().await;

        context.merge_missing(&global_evaluation_context);

        context
    }
}

impl<T> ResolutionDetails<T> {
    fn into_evaluation_details(self, flag_key: impl Into<String>) -> EvaluationDetails<T> {
        EvaluationDetails {
            flag_key: flag_key.into(),
            value: self.value,
            reason: self.reason,
            variant: self.variant,
            flag_metadata: self.flag_metadata.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    //use std::sync::Arc;

    use spec::spec;

    use crate::{
        api::{
            global_evaluation_context::GlobalEvaluationContext, provider_registry::ProviderRegistry,
        },
        provider::{FeatureProvider, MockFeatureProvider, ResolutionDetails},
        Client, EvaluationReason, FlagMetadata, StructValue, Value,
    };

    #[spec(
        number = "1.2.2",
        text = "The client interface MUST define a metadata member or accessor, containing an immutable name field or accessor of type string, which corresponds to the name value supplied during client creation."
    )]
    #[test]
    fn get_metadata_name() {
        assert_eq!(create_default_client().metadata().name, "no_op");
    }

    #[derive(PartialEq, Debug)]
    struct Student {
        id: i64,
        name: String,
    }

    impl TryFrom<StructValue> for Student {
        type Error = String;

        fn try_from(value: StructValue) -> Result<Self, Self::Error> {
            Ok(Student {
                id: value
                    .fields
                    .get("id")
                    .ok_or("id not provided")?
                    .as_i64()
                    .ok_or("id is not a valid number")?,
                name: value
                    .fields
                    .get("name")
                    .ok_or("name not provided")?
                    .as_str()
                    .ok_or("name is not a valid string")?
                    .to_string(),
            })
        }
    }

    #[spec(
        number = "1.3.1.1",
        text = "The client MUST provide methods for typed flag evaluation, including boolean, numeric, string, and structure, with parameters flag key (string, required), default value (boolean | number | string | structure, required), evaluation context (optional), and evaluation options (optional), which returns the flag value."
    )]
    #[spec(
        number = "1.3.3.1",
        text = "The client SHOULD provide functions for floating-point numbers and integers, consistent with language idioms."
    )]
    #[tokio::test]
    async fn get_value() {
        // Test bool.
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});

        provider
            .expect_resolve_bool_value()
            .return_const(Ok(ResolutionDetails::new(true)));

        provider
            .expect_resolve_int_value()
            .return_const(Ok(ResolutionDetails::new(123)));

        provider
            .expect_resolve_float_value()
            .return_const(Ok(ResolutionDetails::new(12.34)));

        provider
            .expect_resolve_string_value()
            .return_const(Ok(ResolutionDetails::new("Hello")));

        provider
            .expect_resolve_struct_value()
            .return_const(Ok(ResolutionDetails::new(
                StructValue::default()
                    .with_field("id", 100)
                    .with_field("name", "Alex"),
            )));

        let client = create_client(provider).await;

        assert_eq!(
            client.get_bool_value("key", None, None).await.unwrap(),
            true
        );

        assert_eq!(client.get_int_value("key", None, None).await.unwrap(), 123);

        assert_eq!(
            client.get_float_value("key", None, None).await.unwrap(),
            12.34
        );

        assert_eq!(
            client.get_string_value("", None, None).await.unwrap(),
            "Hello"
        );

        println!(
            "Result: {:?}",
            client.get_struct_value::<Value>("", None, None).await
        );

        assert_eq!(
            client
                .get_struct_value::<Student>("", None, None)
                .await
                .unwrap(),
            Student {
                id: 100,
                name: "Alex".to_string()
            }
        );
    }

    #[spec(
        number = "1.3.4",
        text = "The client SHOULD guarantee the returned value of any typed flag evaluation method is of the expected type. If the value returned by the underlying provider implementation does not match the expected type, it's to be considered abnormal execution, and the supplied default value should be returned."
    )]
    #[test]
    fn get_value_return_right_type_checked_by_type_system() {}

    #[spec(
        number = "1.4.1.1",
        text = "The client MUST provide methods for detailed flag value evaluation with parameters flag key (string, required), default value (boolean | number | string | structure, required), evaluation context (optional), and evaluation options (optional), which returns an evaluation details structure."
    )]
    #[spec(
        number = "1.4.3",
        text = "The evaluation details structure's value field MUST contain the evaluated flag value."
    )]
    #[spec(
        number = "1.4.4.1",
        text = "The evaluation details structure SHOULD accept a generic argument (or use an equivalent language feature) which indicates the type of the wrapped value field."
    )]
    #[spec(
        number = "1.4.5",
        text = "The evaluation details structure's flag key field MUST contain the flag key argument passed to the detailed flag evaluation method."
    )]
    #[spec(
        number = "1.4.6",
        text = "In cases of normal execution, the evaluation details structure's variant field MUST contain the value of the variant field in the flag resolution structure returned by the configured provider, if the field is set."
    )]
    #[spec(
        number = "1.4.7",
        text = "In cases of normal execution, the evaluation details structure's reason field MUST contain the value of the reason field in the flag resolution structure returned by the configured provider, if the field is set."
    )]
    #[spec(
        number = "1.4.12",
        text = "The client SHOULD provide asynchronous or non-blocking mechanisms for flag evaluation."
    )]
    #[tokio::test]
    async fn get_details() {
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});
        provider
            .expect_resolve_int_value()
            .return_const(Ok(ResolutionDetails::builder()
                .value(123)
                .variant("Static")
                .reason(EvaluationReason::Static)
                .build()));

        let client = create_client(provider).await;

        let result = client.get_int_details("key", None, None).await.unwrap();

        assert_eq!(result.value, 123);
        assert_eq!(result.flag_key, "key");
        assert_eq!(result.reason, Some(EvaluationReason::Static));
        assert_eq!(result.variant, Some("Static".to_string()));
    }

    #[spec(
        number = "1.4.8",
        text = "In cases of abnormal execution, the evaluation details structure's error code field MUST contain an error code."
    )]
    #[spec(
        number = "1.4.9",
        text = "In cases of abnormal execution (network failure, unhandled error, etc) the reason field in the evaluation details SHOULD indicate an error."
    )]
    #[spec(
        number = "1.4.13",
        text = "In cases of abnormal execution, the evaluation details structure's error message field MAY contain a string containing additional details about the nature of the error."
    )]
    #[test]
    fn evaluation_details_contains_error_checked_by_type_system() {}

    #[spec(
        number = "1.4.10",
        text = "Methods, functions, or operations on the client MUST NOT throw exceptions, or otherwise abnormally terminate. Flag evaluation calls must always return the default value in the event of abnormal execution. Exceptions include functions or methods for the purposes for configuration or setup."
    )]
    #[test]
    fn evaluation_return_default_value_covered_by_result() {}

    #[spec(
        number = "1.4.14",
        text = "If the flag metadata field in the flag resolution structure returned by the configured provider is set, the evaluation details structure's flag metadata field MUST contain that value. Otherwise, it MUST contain an empty record."
    )]
    #[spec(
        number = "1.4.14.1",
        text = "Condition: Flag metadata MUST be immutable."
    )]
    #[tokio::test]
    async fn get_details_flag_metadata() {
        let mut provider = MockFeatureProvider::new();
        provider.expect_initialize().returning(|_| {});
        provider
            .expect_resolve_bool_value()
            .return_const(Ok(ResolutionDetails::builder()
                .value(true)
                .flag_metadata(FlagMetadata::default().with_value("Type", "Bool"))
                .build()));

        let client = create_client(provider).await;

        let result = client.get_bool_details("", None, None).await.unwrap();

        assert_eq!(
            *result.flag_metadata.values.get("Type").unwrap(),
            "Bool".into()
        );
    }

    #[spec(
        number = "1.3.2.1",
        text = "The client MUST provide methods for typed flag evaluation, including boolean, numeric, string, and structure, with parameters flag key (string, required), default value (boolean | number | string | structure, required), and evaluation options (optional), which returns the flag value."
    )]
    #[spec(
        number = "1.4.2.1",
        text = "The client MUST provide methods for detailed flag value evaluation with parameters flag key (string, required), default value (boolean | number | string | structure, required), and evaluation options (optional), which returns an evaluation details structure."
    )]
    #[test]
    fn static_context_not_applicable() {}

    fn create_default_client() -> Client {
        Client::new(
            "no_op",
            GlobalEvaluationContext::default(),
            ProviderRegistry::default(),
        )
    }

    async fn create_client(provider: impl FeatureProvider) -> Client {
        let provider_registry = ProviderRegistry::default();
        provider_registry.set_named("custom", provider).await;

        Client::new(
            "custom",
            GlobalEvaluationContext::default(),
            provider_registry,
        )
    }
}
