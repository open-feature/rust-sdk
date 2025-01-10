use crate::{EvaluationContext, EvaluationDetails, EvaluationError, Value};

use super::{Hook, HookContext, HookHints};

use log::Level;

/// A hook that logs the evaluation lifecycle of a flag.
/// See the [spec](https://github.com/open-feature/spec/blob/main/specification/appendix-a-included-utilities.md#logging-hook)
#[derive(Default)]
pub struct LoggingHook {
    pub(crate) include_evaluation_context: bool,
}

#[async_trait::async_trait]
impl Hook for LoggingHook {
    async fn before<'a>(
        &self,
        context: &HookContext<'a>,
        _: Option<&'a HookHints>,
    ) -> Result<Option<EvaluationContext>, EvaluationError> {
        self.log_before(context, Level::Debug);

        Ok(None)
    }
    async fn after<'a>(
        &self,
        context: &HookContext<'a>,
        value: &EvaluationDetails<Value>,
        _: Option<&'a HookHints>,
    ) -> Result<(), EvaluationError> {
        self.log_after(context, value, Level::Debug);

        Ok(())
    }
    async fn error<'a>(
        &self,
        context: &HookContext<'a>,
        error: &EvaluationError,
        _: Option<&'a HookHints>,
    ) {
        self.log_error(context, error);
    }
    async fn finally<'a>(
        &self,
        _: &HookContext<'a>,
        _: &EvaluationDetails<Value>,
        _: Option<&'a HookHints>,
    ) {
    }
}

#[cfg(not(feature = "structured-logging"))]
impl LoggingHook {
    fn log_args(
        &self,
        msg: &str,
        context: &HookContext,
        level: Level,
        additional_args: std::fmt::Arguments,
    ) {
        log::log!(
            level,
            "{}: domain={}, provider_name={}, flag_key={}, default_value={:?}{additional_args}{}",
            msg,
            context.client_metadata.name,
            context.provider_metadata.name,
            context.flag_key,
            context.default_value,
            if self.include_evaluation_context {
                format!(", evaluation_context={:?}", context.evaluation_context)
            } else {
                String::new()
            },
        );
    }

    fn log_before(&self, context: &HookContext, level: Level) {
        self.log_args("Before stage", context, level, format_args!(""));
    }

    fn log_after(&self, context: &HookContext, value: &EvaluationDetails<Value>, level: Level) {
        self.log_args(
            "After stage",
            context,
            level,
            format_args!(
                ", reason={:?}, variant={:?}, value={:?}",
                value.reason, value.variant, value.value
            ),
        );
    }

    fn log_error(&self, context: &HookContext, error: &EvaluationError) {
        self.log_args(
            "Error stage",
            context,
            Level::Error,
            format_args!(", error_message={:?}", error.message),
        );
    }
}

#[cfg(feature = "structured-logging")]
mod structured {
    use super::*;
    use log::{kv::Value as LogValue, Level, Record};

    const DOMAIN_KEY: &str = "domain";
    const PROVIDER_NAME_KEY: &str = "provider_name";
    const FLAG_KEY_KEY: &str = "flag_key";
    const DEFAULT_VALUE_KEY: &str = "default_value";
    const EVALUATION_CONTEXT_KEY: &str = "evaluation_context";
    const ERROR_MESSAGE_KEY: &str = "error_message";
    const REASON_KEY: &str = "reason";
    const VARIANT_KEY: &str = "variant";
    const VALUE_KEY: &str = "value";

    impl LoggingHook {
        fn log_args(
            &self,
            msg: &str,
            context: &HookContext,
            level: Level,
            additional_kvs: Vec<(&str, LogValue)>,
        ) {
            let mut kvs = vec![
                (
                    DOMAIN_KEY,
                    LogValue::from_display(&context.client_metadata.name),
                ),
                (
                    PROVIDER_NAME_KEY,
                    LogValue::from_display(&context.provider_metadata.name),
                ),
                (FLAG_KEY_KEY, LogValue::from_display(&context.flag_key)),
                (
                    DEFAULT_VALUE_KEY,
                    LogValue::from_debug(&context.default_value),
                ),
            ];

            kvs.extend(additional_kvs);

            if self.include_evaluation_context {
                kvs.push((
                    EVALUATION_CONTEXT_KEY,
                    LogValue::from_debug(&context.evaluation_context),
                ));
            }

            let kvs = kvs.as_slice();

            // Single statement to avoid borrowing issues
            // See issue https://github.com/rust-lang/rust/issues/92698
            log::logger().log(
                &Record::builder()
                    .args(format_args!("{}", msg))
                    .level(level)
                    .target("open_feature")
                    .module_path_static(Some(module_path!()))
                    .file_static(Some(file!()))
                    .line(Some(line!()))
                    .key_values(&kvs)
                    .build(),
            );
        }

        pub(super) fn log_before(&self, context: &HookContext, level: Level) {
            self.log_args("Before stage", context, level, vec![]);
        }

        pub(super) fn log_after(
            &self,
            context: &HookContext,
            value: &EvaluationDetails<Value>,
            level: Level,
        ) {
            self.log_args(
                "After stage",
                context,
                level,
                evaluation_details_to_kvs(value),
            );
        }

        pub(super) fn log_error(&self, context: &HookContext, error: &EvaluationError) {
            self.log_args("Error stage", context, Level::Error, error_to_kvs(error));
        }
    }

    fn evaluation_details_to_kvs<'a>(
        details: &'a EvaluationDetails<Value>,
    ) -> Vec<(&'static str, LogValue<'a>)> {
        let kvs = vec![
            (REASON_KEY, LogValue::from_debug(&details.reason)),
            (VARIANT_KEY, LogValue::from_debug(&details.variant)),
            (VALUE_KEY, LogValue::from_debug(&details.value)),
        ];

        kvs
    }

    fn error_to_kvs<'a>(error: &'a EvaluationError) -> Vec<(&'static str, LogValue<'a>)> {
        let kvs = vec![(ERROR_MESSAGE_KEY, LogValue::from_debug(&error.message))];

        kvs
    }
}
