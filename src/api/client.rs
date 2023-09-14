use crate::{EvaluationContext, StructValue};

use super::{
    global_evaluation_context::GlobalEvaluationContext,
    provider_registry::{FeatureProviderWrapper, ProviderRegistry},
};

/// The metadata of OpenFeature client.
pub struct ClientMetadata {
    name: String,
}

/// The OpenFeature client.
/// Create it through the [`OpenFeature`] struct.
pub struct Client {
    pub metadata: ClientMetadata,
    provider_registry: ProviderRegistry,
    evaluation_context: EvaluationContext,
    global_evaluation_context: GlobalEvaluationContext,
}

impl Client {
    pub fn new(
        name: String,
        global_evaluation_context: GlobalEvaluationContext,
        provider_registry: ProviderRegistry,
    ) -> Self {
        Self {
            metadata: ClientMetadata { name },
            global_evaluation_context,
            provider_registry,
            evaluation_context: EvaluationContext::default(),
        }
    }

    pub async fn get_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<&EvaluationContext>,
    ) -> bool {
        let context = self.merge_evaluation_context(evaluation_context).await;

        self.get_provider()
            .await
            .get()
            .resolve_bool_value(flag_key, default_value, &context)
            .await
            .value
    }

    pub async fn get_int_value(
        &self,
        flag_key: &str,
        default_value: i64,
        evaluation_context: Option<&EvaluationContext>,
    ) -> i64 {
        let context = self.merge_evaluation_context(evaluation_context).await;

        self.get_provider()
            .await
            .get()
            .resolve_int_value(flag_key, default_value, &context)
            .await
            .value
    }

    pub async fn get_float_value(
        &self,
        flag_key: &str,
        default_value: f64,
        evaluation_context: Option<&EvaluationContext>,
    ) -> f64 {
        let context = self.merge_evaluation_context(evaluation_context).await;

        self.get_provider()
            .await
            .get()
            .resolve_float_value(flag_key, default_value, &context)
            .await
            .value
    }

    pub async fn get_string_value(
        &self,
        flag_key: &str,
        default_value: &str,
        evaluation_context: Option<&EvaluationContext>,
    ) -> String {
        let context = self.merge_evaluation_context(evaluation_context).await;

        self.get_provider()
            .await
            .get()
            .resolve_string_value(flag_key, default_value, &context)
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
        let context = self.merge_evaluation_context(evaluation_context).await;

        let result = self
            .get_provider()
            .await
            .get()
            .resolve_struct_value(flag_key, StructValue::default(), &context)
            .await;

        if result.is_error() {
            default_value
        } else {
            result.value.into()
        }
    }

    async fn get_provider(&self) -> FeatureProviderWrapper {
        self.provider_registry.get(&self.metadata.name).await
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
