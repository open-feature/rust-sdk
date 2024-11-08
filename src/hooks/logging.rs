use crate::{EvaluationContext, EvaluationDetails, EvaluationError, Value};

use super::{Hook, HookContext, HookHints};

/// A hook that logs the evaluation lifecycle of a flag.
pub struct LoggingHook;

#[async_trait::async_trait]
impl Hook for LoggingHook {
    async fn before<'a>(
        &self,
        context: &HookContext<'a>,
        _: Option<&'a HookHints>,
    ) -> Result<Option<EvaluationContext>, EvaluationError> {
        log::debug!("Before hook for flag {}", context.flag_key);

        Ok(None)
    }
    async fn after<'a>(
        &self,
        context: &HookContext<'a>,
        _: &EvaluationDetails<Value>,
        _: Option<&'a HookHints>,
    ) -> Result<(), EvaluationError> {
        log::debug!("After hook for flag {}", context.flag_key);

        Ok(())
    }
    async fn error<'a>(
        &self,
        context: &HookContext<'a>,
        error: &EvaluationError,
        _: Option<&'a HookHints>,
    ) {
        log::error!("Error hook for flag {}: {:?}", context.flag_key, error);
    }
    async fn finally<'a>(&self, context: &HookContext<'a>, _: Option<&'a HookHints>) {
        log::trace!("Finally hook for flag {}", context.flag_key);
    }
}
