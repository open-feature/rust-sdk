use std::sync::Arc;

use crate::{provider::FeatureProvider, EvaluationContext};

/// The metadata of OpenFeature client.
pub struct ClientMetadata {
    name: String,
}

/// The OpenFeature client.
/// Create it through the [`OpenFeature`] struct.
pub struct Client {
    pub metadata: ClientMetadata,
    provider: Arc<dyn FeatureProvider>,
    evaluation_context: EvaluationContext,
}

impl Client {
    pub fn new(name: String, provider: Arc<dyn FeatureProvider>) -> Self {
        Self {
            metadata: ClientMetadata { name },
            provider,
            evaluation_context: EvaluationContext::default(),
        }
    }

    pub fn with_evaluation_context(&mut self, evaluation_context: EvaluationContext) -> &mut Self {
        self.evaluation_context = evaluation_context;
        self
    }

    pub async fn get_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<EvaluationContext>,
    ) -> bool {
        self.provider
            .resolve_bool_value(flag_key, default_value, evaluation_context)
            .await
            .value
    }
}
