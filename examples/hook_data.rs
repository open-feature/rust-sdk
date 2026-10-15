//! Demonstrates hook data: a timing hook that records a start instant in the `before` stage and
//! reads it back in `finally` to report how long the evaluation took.
//!
//! Hook data is isolated per hook instance and shared across that hook's stages for a single
//! evaluation, which makes it the right place to stash state like a timer or a telemetry span.
//!
//! Run with: `cargo run --example hook_data`

use std::time::Instant;

use open_feature::{
    provider::{FeatureProvider, ProviderMetadata, ProviderStatus, ResolutionDetails},
    EvaluationContext, EvaluationDetails, EvaluationError, EvaluationResult, Hook, HookContext,
    HookHints, OpenFeature, StructValue, Value,
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

/// A hook that times how long a flag evaluation takes by storing an `Instant` in its hook data.
struct TimingHook;

const START_KEY: &str = "start_time";

#[async_trait::async_trait]
impl Hook for TimingHook {
    async fn before<'a>(
        &self,
        context: &HookContext<'a>,
        _hints: Option<&'a HookHints>,
    ) -> Result<Option<EvaluationContext>, EvaluationError> {
        // Stash the start time; this hook data is private to this hook for this evaluation.
        context.data.set(START_KEY, Instant::now());
        Ok(None)
    }

    async fn after<'a>(
        &self,
        _context: &HookContext<'a>,
        _details: &EvaluationDetails<Value>,
        _hints: Option<&'a HookHints>,
    ) -> Result<(), EvaluationError> {
        Ok(())
    }

    async fn error<'a>(
        &self,
        _context: &HookContext<'a>,
        _error: &EvaluationError,
        _hints: Option<&'a HookHints>,
    ) {
    }

    async fn finally<'a>(
        &self,
        context: &HookContext<'a>,
        _evaluation_details: &EvaluationDetails<Value>,
        _hints: Option<&'a HookHints>,
    ) {
        // Consume the same instant we set in `before` and report the elapsed time. `take` gives
        // ownership without requiring the stored value to be `Clone`.
        match context.data.take::<Instant>(START_KEY) {
            Some(start) => log::info!(
                "Flag '{}' evaluated in {:?}",
                context.flag_key,
                start.elapsed()
            ),
            None => log::warn!("Timing hook: no start time recorded"),
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut api = OpenFeature::singleton_mut().await;
    api.set_provider(DummyProvider::default()).await;
    drop(api);

    let client = OpenFeature::singleton()
        .await
        .create_client()
        .with_hook(TimingHook);

    let value = client
        .get_bool_value("my_feature", None, None)
        .await
        .unwrap();

    println!("Feature value: {value}");
}
