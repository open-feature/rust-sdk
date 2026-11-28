use open_feature::provider::{FeatureProvider, ProviderMetadata, ResolutionDetails};
use open_feature::{
    EvaluationContext, EvaluationResult, OpenFeature, StructValue, TrackingEventDetails,
};

/// A provider that records tracking events. A real provider would forward them to an
/// experimentation or analytics backend, typically without blocking the caller.
struct TrackingProvider {
    metadata: ProviderMetadata,
}

impl TrackingProvider {
    fn new() -> Self {
        Self {
            metadata: ProviderMetadata::new("Tracking Provider"),
        }
    }
}

#[async_trait::async_trait]
impl FeatureProvider for TrackingProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    fn track(&self, event_name: &str, context: &EvaluationContext, details: &TrackingEventDetails) {
        println!(
            "tracked '{event_name}' for targeting key {:?} — value: {:?}, fields: {:?}",
            context.targeting_key, details.value, details.values
        );
    }

    async fn resolve_bool_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<bool>> {
        Ok(ResolutionDetails::new(true))
    }

    async fn resolve_int_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<i64>> {
        Ok(ResolutionDetails::new(42))
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<f64>> {
        Ok(ResolutionDetails::new(0.42))
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<String>> {
        Ok(ResolutionDetails::new("42".to_string()))
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<StructValue>> {
        Ok(ResolutionDetails::new(StructValue::default()))
    }
}

#[tokio::main]
async fn main() {
    let mut api = OpenFeature::singleton_mut().await;
    api.set_provider(TrackingProvider::new()).await;
    drop(api);

    let client = OpenFeature::singleton().await.create_client();

    // A tracking event with a scalar value and a custom field. The context passed here is merged
    // with the client and global evaluation contexts before reaching the provider.
    let context = EvaluationContext::default().with_targeting_key("user-123");
    let details = TrackingEventDetails::builder()
        .value(49.99)
        .build()
        .with_field("currency", "USD");

    client
        .track("checkout-completed", Some(&context), Some(details))
        .await;
}
