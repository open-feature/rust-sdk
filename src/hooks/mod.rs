use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{
    provider::ProviderMetadata, ClientMetadata, EvaluationContext, EvaluationDetails,
    EvaluationError, Type, Value,
};

mod data;
pub use data::HookData;

mod logging;
pub use logging::LoggingHook;

// ============================================================
//  Hook
// ============================================================

/// Hook allows application developers to add arbitrary behavior to the flag evaluation lifecycle.
/// They operate similarly to middleware in many web frameworks.
///
/// https://github.com/open-feature/spec/blob/main/specification/sections/04-hooks.md
#[cfg_attr(
    feature = "test-util",
    mockall::automock,
    allow(clippy::ref_option_ref)
)] // Specified lifetimes manually to make it work with mockall
#[async_trait::async_trait]
pub trait Hook: Send + Sync + 'static {
    /// This method is called before the flag evaluation.
    async fn before<'a>(
        &self,
        context: &HookContext<'a>,
        hints: Option<&'a HookHints>,
    ) -> Result<Option<EvaluationContext>, EvaluationError>;

    /// This method is called after the successful flag evaluation.
    async fn after<'a>(
        &self,
        context: &HookContext<'a>,
        details: &EvaluationDetails<Value>,
        hints: Option<&'a HookHints>,
    ) -> Result<(), EvaluationError>;

    /// This method is called on error during flag evaluation or error in before hook or after hook.
    async fn error<'a>(
        &self,
        context: &HookContext<'a>,
        error: &EvaluationError,
        hints: Option<&'a HookHints>,
    );

    /// This method is called after the flag evaluation, regardless of the result.
    async fn finally<'a>(
        &self,
        context: &HookContext<'a>,
        evaluation_details: &EvaluationDetails<Value>,
        hints: Option<&'a HookHints>,
    );
}

// ============================================================
//  HookWrapper
// ============================================================

#[allow(missing_docs)]
#[derive(Clone)]
pub struct HookWrapper(Arc<dyn Hook>);

impl HookWrapper {
    #[allow(missing_docs)]
    pub fn new(hook: impl Hook) -> Self {
        Self(Arc::new(hook))
    }
}

impl Deref for HookWrapper {
    type Target = dyn Hook;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

// ============================================================
//  HookHints
// ============================================================

#[allow(missing_docs)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct HookHints {
    hints: HashMap<String, Value>,
}

// ============================================================
//  HookContext
// ============================================================

/// Context for hooks.
///
/// Implementing a [`Hook`] does not require constructing this type — the SDK builds it and passes a
/// `&HookContext` into each stage. Code that *does* build one with a struct literal (tests, custom
/// hook harnesses) must populate every field, including [`data`](Self::data):
///
/// ```ignore
/// HookContext {
///     // ...other fields...
///     data: HookData::default(),
/// }
/// ```
///
#[allow(missing_docs)]
#[derive(Clone)]
pub struct HookContext<'a> {
    pub flag_key: &'a str,
    pub flag_type: Type,
    pub evaluation_context: &'a EvaluationContext,
    pub provider_metadata: ProviderMetadata,
    pub default_value: Option<Value>,
    pub client_metadata: ClientMetadata,
    /// Mutable, per-hook-instance data shared across this hook's stages within a single evaluation.
    ///
    /// See [`HookData`] for details on lifetime and isolation semantics.
    pub data: HookData,
}

// `HookData` holds type-erased values that are neither comparable nor necessarily `Debug`, so both
// impls deliberately exclude it. Two hook contexts are equal when their immutable fields match.
impl PartialEq for HookContext<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.flag_key == other.flag_key
            && self.flag_type == other.flag_type
            && self.evaluation_context == other.evaluation_context
            && self.provider_metadata == other.provider_metadata
            && self.default_value == other.default_value
            && self.client_metadata == other.client_metadata
    }
}

impl std::fmt::Debug for HookContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HookContext")
            .field("flag_key", &self.flag_key)
            .field("flag_type", &self.flag_type)
            .field("evaluation_context", &self.evaluation_context)
            .field("provider_metadata", &self.provider_metadata)
            .field("default_value", &self.default_value)
            .field("client_metadata", &self.client_metadata)
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(test)]
mod tests {

    use spec::spec;

    use crate::{
        provider::{MockFeatureProvider, ResolutionDetails},
        EvaluationErrorCode, EvaluationOptions, EvaluationReason, OpenFeature, StructValue,
    };

    use super::*;

    #[spec(
        number = "4.1.1",
        text = "Hook context MUST provide: the flag key, flag value type, evaluation context, default value, and hook data."
    )]
    #[spec(
        number = "4.1.2",
        text = "The hook context SHOULD provide: access to the client metadata and the provider metadata fields."
    )]
    #[spec(
        number = "4.1.3",
        text = "The flag key, flag type, and default value properties MUST be immutable. If the language does not support immutability, the hook MUST NOT modify these properties."
    )]
    #[test]
    fn hook_context() {
        let context = HookContext {
            flag_key: "flag_key",
            flag_type: Type::Bool,
            evaluation_context: &EvaluationContext::default(),
            provider_metadata: ProviderMetadata::default(),
            default_value: Some(Value::Bool(true)),
            client_metadata: ClientMetadata::default(),
            data: HookData::default(),
        };

        assert_eq!(context.flag_key, "flag_key");
        assert_eq!(context.flag_type, Type::Bool);
        assert_eq!(context.evaluation_context, &EvaluationContext::default());
        assert_eq!(context.provider_metadata, ProviderMetadata::default());
        assert_eq!(context.default_value, Some(Value::Bool(true)));
        assert_eq!(context.client_metadata, ClientMetadata::default());
        assert!(context.data.is_empty());
    }

    #[spec(
        number = "4.1.5",
        text = "The hook data MUST be mutable. Either the hook data reference itself must be mutable, or it must allow mutation of its contents."
    )]
    #[spec(
        number = "4.6.1",
        text = "hook data MUST be a structure supporting the definition of arbitrary properties, with keys of type string, and values of any type."
    )]
    #[test]
    fn hook_data_arbitrary_mutable_content() {
        // Content is mutable through a shared `&` reference (interior mutability), and values may be
        // of any `'static + Send + Sync` type, not just the restricted `Value` variants.
        let data = HookData::default();
        data.set("string", "value".to_string());
        data.set("int", 42_i64);
        data.set("tuple", (1_i32, "two".to_string()));

        assert_eq!(data.get::<String>("string").as_deref(), Some("value"));
        assert_eq!(data.get::<i64>("int"), Some(42));
        assert_eq!(
            data.get::<(i32, String)>("tuple"),
            Some((1, "two".to_string()))
        );

        // Mutation of existing content is observable.
        data.set("int", 43_i64);
        assert_eq!(data.get::<i64>("int"), Some(43));
    }

    #[spec(
        number = "4.2.1",
        text = "hook hints MUST be a structure supports definition of arbitrary properties, with keys of type string, and values of type boolean | string | number | datetime | structure."
    )]
    #[test]
    fn hook_hints() {
        let mut hints = HookHints::default();
        hints.hints.insert("key".to_string(), Value::Bool(true));
        hints
            .hints
            .insert("key2".to_string(), Value::String("value".to_string()));
        hints.hints.insert("key3".to_string(), Value::Int(42));
        hints.hints.insert("key4".to_string(), Value::Float(3.14));
        hints.hints.insert("key5".to_string(), Value::Array(vec![]));
        hints
            .hints
            .insert("key6".to_string(), Value::Struct(StructValue::default()));

        assert_eq!(hints.hints.len(), 6);
        assert_eq!(hints.hints.get("key"), Some(&Value::Bool(true)));
        assert_eq!(
            hints.hints.get("key2"),
            Some(&Value::String("value".to_string()))
        );
        assert_eq!(hints.hints.get("key3"), Some(&Value::Int(42)));
        assert_eq!(hints.hints.get("key4"), Some(&Value::Float(3.14)));
        assert_eq!(hints.hints.get("key5"), Some(&Value::Array(vec![])));
        assert_eq!(
            hints.hints.get("key6"),
            Some(&Value::Struct(StructValue::default()))
        );
    }

    #[spec(number = "4.2.2.1", text = "Hook hints MUST be immutable.")]
    #[test]
    fn hook_hints_mutability_checked_by_type_system() {}

    #[spec(
        number = "4.2.2.2",
        text = "The client metadata field in the hook context MUST be immutable."
    )]
    #[test]
    fn client_metadata_mutability_checked_by_type_system() {}

    #[spec(
        number = "4.2.2.3",
        text = "The provider metadata field in the hook context MUST be immutable."
    )]
    #[test]
    fn provider_metadata_mutability_checked_by_type_system() {}

    #[spec(number = "4.3.1", text = "Hooks MUST specify at least one stage.")]
    #[test]
    fn hook_interface_implementation_checked_by_type_system() {}

    #[spec(
        number = "4.3.2.1",
        text = "The before stage MUST run before flag resolution occurs. It accepts a hook context (required) and hook hints (optional) as parameters and returns either an evaluation context or nothing."
    )]
    #[test]
    fn hook_before_function_interface_implementation_checked_by_type_system() {}

    #[spec(
        number = "4.3.4",
        text = "Any evaluation context returned from a before hook MUST be passed to subsequent before hooks (via HookContext)."
    )]
    #[tokio::test]
    async fn before_hook_context_passing() {
        let mut mock_hook_1 = MockHook::new();
        let mut mock_hook_2 = MockHook::new();

        let mut api = OpenFeature::default();
        let mut client = api.create_named_client("test");
        let mut mock_provider = MockFeatureProvider::default();

        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .withf(|_, ctx| {
                assert_eq!(
                    ctx,
                    &EvaluationContext::default()
                        .with_targeting_key("mock_hook_1")
                        .with_custom_field("is", "a test")
                );
                true
            })
            .return_const(Ok(ResolutionDetails::new(true)));

        api.set_provider(mock_provider).await;
        drop(api);

        let flag_key = "flag";

        let eval_ctx = EvaluationContext::default().with_custom_field("is", "a test");

        let expected_eval_ctx = eval_ctx.clone();
        let client_metadata = client.metadata().clone();
        mock_hook_1
            .expect_before()
            .withf(move |ctx, _| {
                let hook_ctx_1 = HookContext {
                    flag_key,
                    flag_type: Type::Bool,
                    evaluation_context: &expected_eval_ctx,
                    default_value: Some(Value::Bool(false)),
                    provider_metadata: ProviderMetadata::default(),
                    client_metadata: client_metadata.clone(),
                    data: HookData::default(),
                };

                assert_eq!(ctx, &hook_ctx_1);
                true
            })
            .once()
            .returning(move |_, _| {
                Ok(Some(
                    EvaluationContext::default().with_targeting_key("mock_hook_1"),
                ))
            });

        let expected_eval_ctx_2 = EvaluationContext::default().with_targeting_key("mock_hook_1");
        let client_metadata = client.metadata().clone();
        mock_hook_2
            .expect_before()
            .withf(move |ctx, _| {
                let hook_ctx_1 = HookContext {
                    flag_key,
                    flag_type: Type::Bool,
                    evaluation_context: &expected_eval_ctx_2,
                    default_value: Some(Value::Bool(false)),
                    provider_metadata: ProviderMetadata::default(),
                    client_metadata: client_metadata.clone(),
                    data: HookData::default(),
                };

                assert_eq!(ctx, &hook_ctx_1);
                true
            })
            .once()
            .returning(move |_, _| Ok(None));

        mock_hook_1.expect_after().return_const(Ok(()));
        mock_hook_2.expect_after().return_const(Ok(()));
        mock_hook_1.expect_finally().return_const(());
        mock_hook_2.expect_finally().return_const(());

        // evaluation
        client = client.with_hook(mock_hook_1).with_hook(mock_hook_2);

        let result = client.get_bool_value(flag_key, Some(&eval_ctx), None).await;

        assert!(result.is_ok());
    }

    #[spec(
        number = "4.3.5",
        text = "When before hooks have finished executing, any resulting evaluation context MUST be merged with the existing evaluation context."
    )]
    #[tokio::test]
    async fn before_hook_context_merging() {
        let mut mock_hook = MockHook::new();

        let mut api = OpenFeature::default();
        api.set_evaluation_context(
            EvaluationContext::default()
                .with_custom_field("key", "api context")
                .with_custom_field("lowestPriority", true),
        )
        .await;

        let mut client = api.create_named_client("test");
        client.set_evaluation_context(
            EvaluationContext::default()
                .with_custom_field("key", "client context")
                .with_custom_field("lowestPriority", false)
                .with_custom_field("beatsClient", false),
        );

        mock_hook.expect_before().once().returning(move |_, _| {
            Ok(Some(
                EvaluationContext::default()
                    .with_custom_field("key", "hook value")
                    .with_custom_field("multiplier", 3),
            ))
        });

        mock_hook.expect_after().return_const(Ok(()));
        mock_hook.expect_finally().return_const(());

        let flag_key = "flag";
        let eval_ctx = EvaluationContext::default()
            .with_custom_field("key", "invocation context")
            .with_custom_field("on", true)
            .with_custom_field("beatsClient", true);

        let expected_ctx = EvaluationContext::default()
            .with_custom_field("key", "hook value")
            .with_custom_field("multiplier", 3)
            .with_custom_field("on", true)
            .with_custom_field("lowestPriority", false)
            .with_custom_field("beatsClient", true);

        let mut mock_provider = MockFeatureProvider::default();

        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_string_value()
            .withf(move |_, ctx| {
                assert_eq!(ctx, &expected_ctx);
                true
            })
            .return_const(Ok(ResolutionDetails::new("value")));

        api.set_provider(mock_provider).await;
        drop(api);

        client = client.with_hook(mock_hook);

        let result = client
            .get_string_value(flag_key, Some(&eval_ctx), None)
            .await;

        assert!(result.is_ok());
    }

    #[spec(
        number = "4.3.6",
        text = "The after stage MUST run after flag resolution occurs. It accepts a hook context (required), evaluation details (required) and hook hints (optional). It has no return value."
    )]
    #[tokio::test]
    async fn after_hook() {
        let mut mock_hook = MockHook::new();

        let mut api = OpenFeature::default();
        let mut client = api.create_client();
        let mut mock_provider = MockFeatureProvider::default();

        let mut seq = mockall::Sequence::new();

        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(ResolutionDetails::new(true)));

        api.set_provider(mock_provider).await;
        drop(api);

        mock_hook.expect_before().returning(|_, _| Ok(None));

        mock_hook
            .expect_after()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(()));

        mock_hook.expect_finally().return_const(());

        // evaluation
        client = client.with_hook(mock_hook);

        let flag_key = "flag";
        let eval_ctx = EvaluationContext::default().with_custom_field("is", "a test");

        let result = client.get_bool_value(flag_key, Some(&eval_ctx), None).await;

        assert!(result.is_ok());
    }

    #[spec(
        number = "4.3.7",
        text = "The error hook MUST run when errors are encountered in the before stage, the after stage or during flag resolution. It accepts hook context (required), exception representing what went wrong (required), and hook hints (optional). It has no return value."
    )]
    #[tokio::test]
    async fn error_hook() {
        // error on `before` hook
        {
            let mut mock_hook = MockHook::new();

            let mut api = OpenFeature::default();
            let mut client = api.create_client();
            let mut mock_provider = MockFeatureProvider::default();

            let mut seq = mockall::Sequence::new();

            mock_provider.expect_hooks().return_const(vec![]);
            mock_provider.expect_initialize().return_const(());
            mock_provider.expect_resolve_bool_value().never();
            mock_provider
                .expect_metadata()
                .return_const(ProviderMetadata::default());

            api.set_provider(mock_provider).await;
            drop(api);

            mock_hook.expect_before().returning(|_, _| error());

            mock_hook
                .expect_error()
                .once()
                .in_sequence(&mut seq)
                .return_const(());

            mock_hook
                .expect_finally()
                .withf(|ctx, details, _| {
                    assert_eq!(ctx.flag_key, "flag");
                    assert_eq!(ctx.flag_type, Type::Bool);
                    assert_eq!(
                        ctx.evaluation_context,
                        &EvaluationContext::default().with_custom_field("is", "a test")
                    );
                    assert_eq!(ctx.default_value, Some(Value::Bool(false)));
                    assert_eq!(details.flag_key, "flag");
                    assert_eq!(details.value, Value::Bool(false));
                    assert_eq!(details.reason, Some(EvaluationReason::Error));
                    true
                })
                .return_const(());

            // evaluation
            client = client.with_hook(mock_hook);

            let flag_key = "flag";
            let eval_ctx = EvaluationContext::default().with_custom_field("is", "a test");

            let result = client.get_bool_value(flag_key, Some(&eval_ctx), None).await;

            assert!(result.is_err());
        }

        // error on evaluation
        {
            let mut mock_hook = MockHook::new();

            let mut api = OpenFeature::default();
            let mut client = api.create_client();
            let mut mock_provider = MockFeatureProvider::default();

            let mut seq = mockall::Sequence::new();

            mock_provider.expect_hooks().return_const(vec![]);
            mock_provider.expect_initialize().return_const(());
            mock_provider
                .expect_metadata()
                .return_const(ProviderMetadata::default());

            mock_hook.expect_before().returning(|_, _| Ok(None));

            mock_provider
                .expect_resolve_bool_value()
                .once()
                .in_sequence(&mut seq)
                .return_const(error());

            mock_hook
                .expect_error()
                .once()
                .in_sequence(&mut seq)
                .return_const(());

            mock_hook.expect_finally().return_const(());

            api.set_provider(mock_provider).await;
            drop(api);

            // evaluation
            client = client.with_hook(mock_hook);

            let flag_key = "flag";
            let eval_ctx = EvaluationContext::default().with_custom_field("is", "a test");

            let result = client.get_bool_value(flag_key, Some(&eval_ctx), None).await;

            assert!(result.is_err());
        }
    }

    #[spec(
        number = "4.3.8",
        text = "The finally hook MUST run after the before, after, and error stages. It accepts a hook context (required), evaluation details (required) and hook hints (optional). It has no return value."
    )]
    #[tokio::test]
    async fn finally_hook() {
        let mut mock_hook = MockHook::new();

        let mut api = OpenFeature::default();
        let mut client = api.create_client();
        let mut mock_provider = MockFeatureProvider::default();

        let mut seq = mockall::Sequence::new();

        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .return_const(Ok(ResolutionDetails::new(true)));

        api.set_provider(mock_provider).await;

        mock_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(None));
        mock_hook
            .expect_after()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(()));

        mock_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .withf(|ctx, details, _| {
                assert_eq!(ctx.flag_key, "flag");
                assert_eq!(ctx.flag_type, Type::Bool);
                assert_eq!(
                    ctx.evaluation_context,
                    &EvaluationContext::default().with_custom_field("is", "a test")
                );
                assert_eq!(ctx.default_value, Some(Value::Bool(false)));
                assert_eq!(details.flag_key, "flag");
                assert_eq!(details.value, Value::Bool(true));
                true
            })
            .return_const(());

        // evaluation
        client = client.with_hook(mock_hook);

        let flag_key = "flag";
        let eval_ctx = EvaluationContext::default().with_custom_field("is", "a test");

        let result = client.get_bool_value(flag_key, Some(&eval_ctx), None).await;

        assert!(result.is_ok());
    }

    #[spec(
        number = "4.4.1",
        text = "The API, Client, Provider, and invocation MUST have a method for registering hooks."
    )]
    #[spec(
        number = "4.4.2",
        text = "Hooks MUST be evaluated in the following order -> before: API, Client, Invocation, Provider. after: Provider, Invocation, Client, API. error(if applicable): Provider, Invocation, Client, API. finally: Provider, Invocation, Client, API."
    )]
    #[tokio::test]
    async fn hook_evaluation_order() {
        let mut mock_api_hook = MockHook::new();
        let mut mock_client_hook = MockHook::new();
        let mut mock_provider_hook = MockHook::new();
        let mut mock_invocation_hook = MockHook::new();

        let mut api = OpenFeature::default();
        let mut client = api.create_client();
        let mut provider = MockFeatureProvider::default();

        let mut seq = mockall::Sequence::new();

        // before: API, Client, Invocation, Provider
        mock_api_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(None));
        mock_client_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(None));
        mock_invocation_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(None));
        mock_provider_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(None));

        // evaluation
        provider
            .expect_resolve_bool_value()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(ResolutionDetails::new(true)));

        // after: Provider, Invocation, Client, API
        mock_provider_hook
            .expect_after()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| Ok(()));
        mock_invocation_hook
            .expect_after()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| Ok(()));
        mock_client_hook
            .expect_after()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| Ok(()));
        mock_api_hook
            .expect_after()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| Ok(()));

        // finally: Provider, Invocation, Client, API
        mock_provider_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_invocation_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_client_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_api_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});

        provider
            .expect_hooks()
            .return_const(vec![HookWrapper::new(mock_provider_hook)]);
        provider.expect_initialize().return_const(());
        provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());

        api.set_provider(provider).await;
        api.add_hook(mock_api_hook).await;
        client = client.with_hook(mock_client_hook);

        let eval = EvaluationOptions::default().with_hook(mock_invocation_hook);
        let _ = client.get_bool_value("flag", None, Some(&eval)).await;
    }

    #[spec(
        number = "4.4.3",
        text = "If a finally hook abnormally terminates, evaluation MUST proceed, including the execution of any remaining finally hooks."
    )]
    #[test]
    fn finally_hook_not_throw_checked_by_type_system() {}

    #[spec(
        number = "4.4.4",
        text = "If an error hook abnormally terminates, evaluation MUST proceed, including the execution of any remaining error hooks."
    )]
    #[test]
    fn error_hook_not_throw_checked_by_type_system() {}

    #[spec(
        number = "4.4.5",
        text = "If an error occurs in the before or after hooks, the error hooks MUST be invoked."
    )]
    #[tokio::test]
    async fn error_hook_invoked_on_error() {
        let mut mock_hook = MockHook::new();

        let mut api = OpenFeature::default();
        let mut client = api.create_client();
        let mut mock_provider = MockFeatureProvider::default();

        let mut seq = mockall::Sequence::new();

        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider.expect_resolve_bool_value().never();
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());

        api.set_provider(mock_provider).await;

        mock_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| error());

        mock_hook
            .expect_error()
            .once()
            .in_sequence(&mut seq)
            .return_const(());

        mock_hook.expect_finally().return_const(());

        // evaluation
        client = client.with_hook(mock_hook);

        let flag_key = "flag";
        let eval_ctx = EvaluationContext::default().with_custom_field("is", "a test");

        let result = client.get_bool_value(flag_key, Some(&eval_ctx), None).await;

        assert!(result.is_err());
    }

    #[spec(
        number = "4.4.6",
        text = "If an error occurs during the evaluation of before or after hooks, any remaining hooks in the before or after stages MUST NOT be invoked."
    )]
    #[tokio::test]
    async fn do_not_evaluate_remaining_hooks_on_error() {
        let mut mock_api_hook = MockHook::new();
        let mut mock_client_hook = MockHook::new();
        let mut mock_provider_hook = MockHook::new();
        let mut mock_invocation_hook = MockHook::new();

        let mut api = OpenFeature::default();
        let mut client = api.create_client();
        let mut provider = MockFeatureProvider::default();

        let mut seq = mockall::Sequence::new();

        // before: API, Client, Invocation, Provider
        mock_api_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| Ok(None));
        mock_client_hook
            .expect_before()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _| error());

        // Remaining `before` and `after` hooks should not be called
        mock_invocation_hook.expect_before().never();
        mock_provider_hook.expect_before().never();

        // evaluation should not be called
        provider.expect_resolve_bool_value().never();

        // after: Provider, Invocation, Client, API
        mock_provider_hook.expect_after().never();
        mock_invocation_hook.expect_after().never();
        mock_client_hook.expect_after().never();
        mock_api_hook.expect_after().never();

        // error: Provider, Invocation, Client, API
        mock_provider_hook
            .expect_error()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_invocation_hook
            .expect_error()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_client_hook
            .expect_error()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_api_hook
            .expect_error()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});

        // finally: Provider, Invocation, Client, API
        mock_provider_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_invocation_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_client_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});
        mock_api_hook
            .expect_finally()
            .once()
            .in_sequence(&mut seq)
            .returning(|_, _, _| {});

        provider
            .expect_hooks()
            .return_const(vec![HookWrapper::new(mock_provider_hook)]);
        provider.expect_initialize().return_const(());
        provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());

        api.set_provider(provider).await;
        api.add_hook(mock_api_hook).await;
        client = client.with_hook(mock_client_hook);

        let eval = EvaluationOptions::default().with_hook(mock_invocation_hook);
        let result = client.get_bool_value("flag", None, Some(&eval)).await;

        assert!(result.is_err());
    }

    #[spec(
        number = "4.4.7",
        text = "If an error occurs in the before hooks, the default value MUST be returned."
    )]
    #[test]
    fn default_value_covered_by_implementing_default_trait() {}

    fn error<T>() -> Result<T, EvaluationError> {
        Err(EvaluationError {
            code: EvaluationErrorCode::General("error".to_string()),
            message: None,
        })
    }

    #[spec(
        number = "4.3.2",
        text = "Hook data MUST be created before the first stage invoked in a hook for a specific evaluation and propagated between each stage of the hook. The hook data is not shared between different hooks."
    )]
    #[tokio::test]
    async fn hook_data_shared_across_stages_success() {
        struct StateSharingHook {
            stages_run: Arc<std::sync::Mutex<Vec<String>>>,
        }

        #[async_trait::async_trait]
        impl Hook for StateSharingHook {
            async fn before<'a>(
                &self,
                context: &HookContext<'a>,
                _hints: Option<&'a HookHints>,
            ) -> Result<Option<EvaluationContext>, EvaluationError> {
                context
                    .data
                    .set("shared_key", "hello from before".to_string());
                self.stages_run.lock().unwrap().push("before".to_string());
                Ok(None)
            }

            async fn after<'a>(
                &self,
                context: &HookContext<'a>,
                _details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) -> Result<(), EvaluationError> {
                let val = context.data.get::<String>("shared_key");
                assert_eq!(val.as_deref(), Some("hello from before"));
                self.stages_run.lock().unwrap().push("after".to_string());
                Ok(())
            }

            async fn error<'a>(
                &self,
                context: &HookContext<'a>,
                _error: &EvaluationError,
                _hints: Option<&'a HookHints>,
            ) {
                let val = context.data.get::<String>("shared_key");
                assert_eq!(val.as_deref(), Some("hello from before"));
                self.stages_run.lock().unwrap().push("error".to_string());
            }

            async fn finally<'a>(
                &self,
                context: &HookContext<'a>,
                _evaluation_details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) {
                let val = context.data.get::<String>("shared_key");
                assert_eq!(val.as_deref(), Some("hello from before"));
                self.stages_run.lock().unwrap().push("finally".to_string());
            }
        }

        let stages = Arc::new(std::sync::Mutex::new(Vec::new()));
        let hook = StateSharingHook {
            stages_run: Arc::clone(&stages),
        };

        let mut api = OpenFeature::default();
        let client = api.create_named_client("test").with_hook(hook);

        let mut mock_provider = MockFeatureProvider::default();
        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .return_const(Ok(ResolutionDetails::new(true)));

        api.set_provider(mock_provider).await;
        drop(api);

        let result = client.get_bool_value("flag", None, None).await;
        assert!(result.is_ok());

        let stages_run = stages.lock().unwrap();
        assert_eq!(*stages_run, vec!["before", "after", "finally"]);
    }

    #[spec(
        number = "4.3.2",
        text = "Hook data MUST be created before the first stage invoked in a hook for a specific evaluation and propagated between each stage of the hook. The hook data is not shared between different hooks."
    )]
    #[tokio::test]
    async fn hook_data_shared_across_stages_error() {
        struct StateSharingHook {
            stages_run: Arc<std::sync::Mutex<Vec<String>>>,
        }

        #[async_trait::async_trait]
        impl Hook for StateSharingHook {
            async fn before<'a>(
                &self,
                context: &HookContext<'a>,
                _hints: Option<&'a HookHints>,
            ) -> Result<Option<EvaluationContext>, EvaluationError> {
                context.data.set("shared_key", "error context".to_string());
                self.stages_run.lock().unwrap().push("before".to_string());
                Ok(None)
            }

            async fn after<'a>(
                &self,
                context: &HookContext<'a>,
                _details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) -> Result<(), EvaluationError> {
                let val = context.data.get::<String>("shared_key");
                assert_eq!(val.as_deref(), Some("error context"));
                self.stages_run.lock().unwrap().push("after".to_string());
                Ok(())
            }

            async fn error<'a>(
                &self,
                context: &HookContext<'a>,
                _error: &EvaluationError,
                _hints: Option<&'a HookHints>,
            ) {
                let val = context.data.get::<String>("shared_key");
                assert_eq!(val.as_deref(), Some("error context"));
                self.stages_run.lock().unwrap().push("error".to_string());
            }

            async fn finally<'a>(
                &self,
                context: &HookContext<'a>,
                _evaluation_details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) {
                let val = context.data.get::<String>("shared_key");
                assert_eq!(val.as_deref(), Some("error context"));
                self.stages_run.lock().unwrap().push("finally".to_string());
            }
        }

        let stages = Arc::new(std::sync::Mutex::new(Vec::new()));
        let hook = StateSharingHook {
            stages_run: Arc::clone(&stages),
        };

        let mut api = OpenFeature::default();
        let client = api.create_named_client("test").with_hook(hook);

        let mut mock_provider = MockFeatureProvider::default();
        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .return_const(Err(EvaluationError {
                code: EvaluationErrorCode::FlagNotFound,
                message: Some("flag not found".to_string()),
            }));

        api.set_provider(mock_provider).await;
        drop(api);

        let result = client.get_bool_value("flag", None, None).await;
        assert!(result.is_err());

        let stages_run = stages.lock().unwrap();
        assert_eq!(*stages_run, vec!["before", "error", "finally"]);
    }

    #[spec(
        number = "4.3.2",
        text = "Hook data MUST be created before the first stage invoked in a hook for a specific evaluation and propagated between each stage of the hook. The hook data is not shared between different hooks."
    )]
    #[tokio::test]
    async fn hook_data_isolated_between_hooks() {
        // Each hook writes its own `id` under the SAME key in `before`, then in `after` asserts it
        // reads back exactly what it wrote — proving hooks do not share a hook data instance.
        struct IsolationHook {
            id: &'static str,
            violations: Arc<std::sync::Mutex<Vec<String>>>,
            checks: Arc<std::sync::atomic::AtomicUsize>,
        }

        #[async_trait::async_trait]
        impl Hook for IsolationHook {
            async fn before<'a>(
                &self,
                context: &HookContext<'a>,
                _hints: Option<&'a HookHints>,
            ) -> Result<Option<EvaluationContext>, EvaluationError> {
                context.data.set("owner", self.id.to_string());
                Ok(None)
            }

            async fn after<'a>(
                &self,
                context: &HookContext<'a>,
                _details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) -> Result<(), EvaluationError> {
                // A leak from another hook would surface a different owner here.
                if context.data.get::<String>("owner").as_deref() != Some(self.id) {
                    self.violations.lock().unwrap().push(self.id.to_string());
                }
                self.checks
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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
                _context: &HookContext<'a>,
                _evaluation_details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) {
            }
        }

        let violations = Arc::new(std::sync::Mutex::new(Vec::new()));
        let checks = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let mut api = OpenFeature::default();
        let client = api
            .create_named_client("test")
            .with_hook(IsolationHook {
                id: "hook_a",
                violations: Arc::clone(&violations),
                checks: Arc::clone(&checks),
            })
            .with_hook(IsolationHook {
                id: "hook_b",
                violations: Arc::clone(&violations),
                checks: Arc::clone(&checks),
            });

        let mut mock_provider = MockFeatureProvider::default();
        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .return_const(Ok(ResolutionDetails::new(true)));

        api.set_provider(mock_provider).await;
        drop(api);

        let result = client.get_bool_value("flag", None, None).await;
        assert!(result.is_ok());

        // Without this the test would pass vacuously: `violations` is also empty when no hook ran.
        assert_eq!(
            checks.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "expected both hooks to reach `after` and check their data"
        );
        let violations = violations.lock().unwrap();
        assert!(
            violations.is_empty(),
            "hooks observed another hook's data: {violations:?}"
        );
    }

    #[spec(
        number = "4.3.2",
        text = "Hook data MUST be created before the first stage invoked in a hook for a specific evaluation and propagated between each stage of the hook. The hook data is not shared between different hooks."
    )]
    #[tokio::test]
    async fn hook_data_not_shared_between_evaluations() {
        // The SAME hook instance is reused across evaluations, but each evaluation must start with
        // a fresh, empty `HookData`. A `before` stage that observes a marker left by a previous
        // evaluation would prove data leaked across evaluations.
        struct PerEvaluationHook {
            leaked: Arc<std::sync::atomic::AtomicBool>,
            evaluations: Arc<std::sync::atomic::AtomicUsize>,
        }

        #[async_trait::async_trait]
        impl Hook for PerEvaluationHook {
            async fn before<'a>(
                &self,
                context: &HookContext<'a>,
                _hints: Option<&'a HookHints>,
            ) -> Result<Option<EvaluationContext>, EvaluationError> {
                self.evaluations
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                // Nothing from a prior evaluation should be visible here.
                if context.data.contains_key("marker") {
                    self.leaked.store(true, std::sync::atomic::Ordering::SeqCst);
                }
                context.data.set("marker", true);
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
                _context: &HookContext<'a>,
                _evaluation_details: &EvaluationDetails<Value>,
                _hints: Option<&'a HookHints>,
            ) {
            }
        }

        let leaked = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let evaluations = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let mut api = OpenFeature::default();
        let client = api
            .create_named_client("test")
            .with_hook(PerEvaluationHook {
                leaked: Arc::clone(&leaked),
                evaluations: Arc::clone(&evaluations),
            });

        let mut mock_provider = MockFeatureProvider::default();
        mock_provider.expect_hooks().return_const(vec![]);
        mock_provider.expect_initialize().return_const(());
        mock_provider
            .expect_metadata()
            .return_const(ProviderMetadata::default());
        mock_provider
            .expect_resolve_bool_value()
            .return_const(Ok(ResolutionDetails::new(true)));

        api.set_provider(mock_provider).await;
        drop(api);

        // Two evaluations with the same client (and thus the same hook instance).
        assert!(client.get_bool_value("flag", None, None).await.is_ok());
        assert!(client.get_bool_value("flag", None, None).await.is_ok());

        assert_eq!(
            evaluations.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "expected the hook to run for both evaluations"
        );
        assert!(
            !leaked.load(std::sync::atomic::Ordering::SeqCst),
            "hook data leaked from one evaluation into the next"
        );
    }
}
