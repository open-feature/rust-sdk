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

pub trait FromStructValue<Out = Self> {
    fn from_struct_value(value: &StructValue) -> anyhow::Result<Out>;
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
    pub async fn get_struct_value<T: FromStructValue>(
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

        match T::from_struct_value(&result.value) {
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
            .to_evaluation_details(flag_key))
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
            .to_evaluation_details(flag_key))
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
            .to_evaluation_details(flag_key))
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
            .to_evaluation_details(flag_key))
    }

    /// Return the [`EvaluationDetails`] with given `flag_key`, `evaluation_context` and
    /// `evaluation_options`.
    #[allow(unused_variables)]
    pub async fn get_struct_details<T: FromStructValue>(
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

        match T::from_struct_value(&result.value) {
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
    fn to_evaluation_details(self, flag_key: impl Into<String>) -> EvaluationDetails<T> {
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
    use std::sync::Arc;

    use spec::spec;

    use crate::{
        api::{
            global_evaluation_context::GlobalEvaluationContext, provider_registry::ProviderRegistry,
        },
        provider::NoOpProvider,
        Client, EvaluationReason, FlagMetadata, StructValue,
    };

    use super::FromStructValue;

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

    impl FromStructValue for Student {
        fn from_struct_value(value: &StructValue) -> anyhow::Result<Self> {
            Ok(Student {
                id: value.fields.get("id").unwrap().as_i64().unwrap(),
                name: value
                    .fields
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            })
        }
    }

    #[spec(
        number = "1.3.1.1",
        text = "The client MUST provide methods for typed flag evaluation, including boolean, numeric, string, and structure, with parameters flag key (string, required), default value (boolean | number | string | structure, required), evaluation context (optional), and evaluation options (optional), which returns the flag value."
    )]
    #[tokio::test]
    async fn get_value() {
        // Test bool.
        let client = create_client(NoOpProvider::builder().bool_value(true).build()).await;

        assert_eq!(
            client.get_bool_value("key", None, None).await.unwrap(),
            true
        );

        // Test string.
        let client = create_client(NoOpProvider::builder().string_value("result").build()).await;

        assert_eq!(
            client.get_string_value("", None, None).await.unwrap(),
            "result"
        );

        // Test struct.
        let client = create_client(
            NoOpProvider::builder()
                .struct_value(Arc::new(
                    StructValue::default()
                        .with_field("id", 100)
                        .with_field("name", "Alex"),
                ))
                .build(),
        )
        .await;

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
        number = "1.3.3.1",
        text = "The client SHOULD provide functions for floating-point numbers and integers, consistent with language idioms."
    )]
    #[tokio::test]
    async fn get_numeric_value() {
        // Test int.
        let client = create_client(NoOpProvider::builder().int_value(200).build()).await;
        assert_eq!(client.get_int_value("key", None, None).await.unwrap(), 200);

        // Test float.
        let client = create_client(NoOpProvider::builder().float_value(5.0).build()).await;
        assert_eq!(client.get_float_value("", None, None).await.unwrap(), 5.0);
    }

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
        let provider = NoOpProvider::builder().int_value(100).build();
        let client = create_client(provider).await;

        let result = client.get_int_details("key", None, None).await.unwrap();

        assert_eq!(result.value, 100);
        assert_eq!(result.flag_key, "key");
        assert_eq!(result.reason, Some(EvaluationReason::Static));
        assert_eq!(result.variant, Some("Static".to_string()));

        assert_eq!(
            client
                .get_bool_details("another_key", None, None)
                .await
                .unwrap()
                .reason,
            Some(EvaluationReason::Default)
        );
    }

    #[spec(
        number = "1.4.14",
        text = "If the flag metadata field in the flag resolution structure returned by the configured provider is set, the evaluation details structure's flag metadata field MUST contain that value. Otherwise, it MUST contain an empty record."
    )]
    #[tokio::test]
    async fn get_details_flag_metadata() {
        let client = create_default_client();

        let result = client.get_bool_details("", None, None).await.unwrap();
        assert_eq!(
            *result.flag_metadata.values.get("Type").unwrap(),
            "Bool".into()
        );

        assert_eq!(
            client
                .get_struct_details::<Student>("", None, None)
                .await
                .unwrap()
                .flag_metadata,
            FlagMetadata::default()
        )
    }

    fn create_default_client() -> Client {
        Client::new(
            "no_op",
            GlobalEvaluationContext::default(),
            ProviderRegistry::default(),
        )
    }

    async fn create_client(provider: NoOpProvider) -> Client {
        let provider_registry = ProviderRegistry::default();
        provider_registry.set_named("custom", provider).await;

        Client::new(
            "custom",
            GlobalEvaluationContext::default(),
            provider_registry,
        )
    }
}
