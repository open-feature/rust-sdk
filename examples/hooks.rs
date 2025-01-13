use open_feature::{
    provider::{FeatureProvider, ProviderMetadata, ProviderStatus, ResolutionDetails},
    EvaluationContext, EvaluationDetails, EvaluationError, EvaluationOptions, EvaluationResult,
    Hook, HookContext, HookHints, OpenFeature, StructValue, Value,
};

struct DummyProvider(ProviderMetadata);

impl Default for DummyProvider {
    fn default() -> Self {
        Self(ProviderMetadata::new("Dummy Provider"))
    }
}

#[async_trait::async_trait]
impl FeatureProvider for DummyProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.0
    }

    fn status(&self) -> ProviderStatus {
        ProviderStatus::Ready
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
        unimplemented!()
    }

    async fn resolve_float_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<f64>> {
        unimplemented!()
    }

    async fn resolve_string_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<String>> {
        unimplemented!()
    }

    async fn resolve_struct_value(
        &self,
        _flag_key: &str,
        _evaluation_context: &EvaluationContext,
    ) -> Result<ResolutionDetails<StructValue>, EvaluationError> {
        unimplemented!()
    }
}

struct DummyLoggingHook(String);

#[async_trait::async_trait]
impl Hook for DummyLoggingHook {
    async fn before<'a>(
        &self,
        context: &HookContext<'a>,
        _hints: Option<&'a HookHints>,
    ) -> Result<Option<EvaluationContext>, EvaluationError> {
        log::info!(
            "Evaluating({}) flag {} of type {}",
            self.0,
            context.flag_key,
            context.flag_type
        );

        Ok(None)
    }

    async fn after<'a>(
        &self,
        context: &HookContext<'a>,
        details: &EvaluationDetails<Value>,
        _hints: Option<&'a HookHints>,
    ) -> Result<(), EvaluationError> {
        log::info!(
            "Flag({}) {} of type {} evaluated to {:?}",
            self.0,
            context.flag_key,
            context.flag_type,
            details.value
        );

        Ok(())
    }

    async fn error<'a>(
        &self,
        context: &HookContext<'a>,
        error: &EvaluationError,
        _hints: Option<&'a HookHints>,
    ) {
        log::error!(
            "Error({}) evaluating flag {} of type {}: {:?}",
            self.0,
            context.flag_key,
            context.flag_type,
            error
        );
    }

    async fn finally<'a>(
        &self,
        context: &HookContext<'a>,
        _: &EvaluationDetails<Value>,
        _hints: Option<&'a HookHints>,
    ) {
        log::info!(
            "Finally({}) evaluating flag {} of type {}",
            self.0,
            context.flag_key,
            context.flag_type
        );
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut api = OpenFeature::singleton_mut().await;
    api.set_provider(DummyProvider::default()).await;
    api.add_hook(DummyLoggingHook("global".to_string())).await;
    drop(api);

    let client = OpenFeature::singleton()
        .await
        .create_client()
        .with_hook(DummyLoggingHook("client".to_string())); // Add a client-level hook

    let eval = EvaluationOptions::default().with_hook(DummyLoggingHook("eval".to_string()));
    let feature = client
        .get_bool_details("my_feature", None, Some(&eval))
        .await
        .unwrap();

    println!("Feature value: {}", feature.value);
}
