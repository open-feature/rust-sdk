use crate::{EvaluationContext, StructValue};

use super::provider_registry::{self, FeatureProviderWrapper, ProviderRegistry};

/// The metadata of OpenFeature client.
pub struct ClientMetadata {
    name: String,
}

/// The OpenFeature client.
/// Create it through the [`OpenFeature`] struct.
pub struct Client {
    pub metadata: ClientMetadata,
    providers: ProviderRegistry,
    evaluation_context: EvaluationContext,
}

impl Client {
    pub fn new(name: String, providers: ProviderRegistry) -> Self {
        Self {
            metadata: ClientMetadata { name },
            providers,
            evaluation_context: EvaluationContext::default(),
        }
    }

    pub async fn get_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<&EvaluationContext>,
    ) -> bool {
        self.get_provider()
            .await
            .get()
            .resolve_bool_value(flag_key, default_value, evaluation_context)
            .await
            .value
    }

    pub async fn get_int_value(
        &self,
        flag_key: &str,
        default_value: i64,
        evaluation_context: Option<&EvaluationContext>,
    ) -> i64 {
        self.get_provider()
            .await
            .get()
            .resolve_int_value(flag_key, default_value, evaluation_context)
            .await
            .value
    }

    pub async fn get_float_value(
        &self,
        flag_key: &str,
        default_value: f64,
        evaluation_context: Option<&EvaluationContext>,
    ) -> f64 {
        self.get_provider()
            .await
            .get()
            .resolve_float_value(flag_key, default_value, evaluation_context)
            .await
            .value
    }

    pub async fn get_string_value(
        &self,
        flag_key: &str,
        default_value: &str,
        evaluation_context: Option<&EvaluationContext>,
    ) -> String {
        self.get_provider()
            .await
            .get()
            .resolve_string_value(flag_key, default_value, evaluation_context)
            .await
            .value
    }

    pub async fn get_struct_value<T>(
        &self,
        flag_key: &str,
        default_value: T,
        evaluation_context: Option<&EvaluationContext>,
    ) -> T
    where
        T: From<StructValue>,
    {
        let result = self
            .get_provider()
            .await
            .get()
            .resolve_struct_value(flag_key, StructValue::default(), evaluation_context)
            .await;

        if result.is_error() {
            default_value
        } else {
            result.value.into()
        }
    }

    async fn get_provider(&self) -> FeatureProviderWrapper {
        self.providers.get(&self.metadata.name).await
    }
}
