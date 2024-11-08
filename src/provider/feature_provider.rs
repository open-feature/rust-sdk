use async_trait::async_trait;

use crate::{EvaluationContext, EvaluationResult, StructValue};

use super::ResolutionDetails;

// ============================================================
//  FeatureProvider
// ============================================================

/// This trait defines interfaces that Provider Authors can use to abstract a particular flag
/// management system, thus enabling the use of the evaluation API by Application Authors.
///
/// Providers are the "translator" between the flag evaluation calls made in application code, and
/// the flag management system that stores flags and in some cases evaluates flags. At a minimum,
/// providers should implement some basic evaluation methods which return flag values of the
/// expected type. In addition, providers may transform the evaluation context appropriately in
/// order to be used in dynamic evaluation of their associated flag management system, provide
/// insight into why evaluation proceeded the way it did, and expose configuration options for
/// their associated flag management system. Hypothetical provider implementations might wrap a
/// vendor SDK, embed an REST client, or read flags from a local file.
///
/// See the [spec](https://openfeature.dev/specification/sections/providers).
#[cfg_attr(feature = "test-util", mockall::automock)]
#[async_trait]
pub trait FeatureProvider: Send + Sync + 'static {
    /// The provider MAY define an initialize function which accepts the global evaluation
    /// context as an argument and performs initialization logic relevant to the provider.
    ///
    /// Note the following rules:
    /// * The provider MUST set its status field/accessor to READY if its initialize function
    /// terminates normally.
    /// * The provider MUST set its status field to ERROR if its initialize function terminates
    /// abnormally.
    /// * The provider SHOULD indicate an error if flag resolution is attempted before the provider
    /// is ready.
    #[allow(unused_variables)]
    async fn initialize(&mut self, context: &EvaluationContext) {}

    /// The provider MAY define a status field/accessor which indicates the readiness of the
    /// provider, with possible values NOT_READY, READY, or ERROR.
    ///
    /// Providers without this field can be assumed to be ready immediately.
    fn status(&self) -> ProviderStatus {
        ProviderStatus::Ready
    }

    /// The provider interface MUST define a metadata member or accessor, containing a name field
    /// or accessor of type string, which identifies the provider implementation.
    fn metadata(&self) -> &ProviderMetadata;

    /// The provider MAY define a hooks field or accessor which returns a list of hooks that
    /// the provider supports.
    fn hooks(&self) -> &[crate::hooks::HookWrapper] {
        &[]
    }

    /// Resolve given `flag_key` as a bool value.
    async fn resolve_bool_value(
        &self,
        flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<bool>>;

    /// Resolve given `flag_key` as an i64 value.
    async fn resolve_int_value(
        &self,
        flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<i64>>;

    /// Resolve given `flag_key` as a f64 value.
    async fn resolve_float_value(
        &self,
        flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<f64>>;

    /// Resolve given `flag_key` as a string value.
    async fn resolve_string_value(
        &self,
        flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<String>>;

    /// Resolve given `flag_key` as a struct value.
    async fn resolve_struct_value(
        &self,
        flag_key: &str,
        evaluation_context: &EvaluationContext,
    ) -> EvaluationResult<ResolutionDetails<StructValue>>;
}

// ============================================================
//  ProviderMetadata
// ============================================================

/// The metadata of a feature provider.
#[derive(Clone, Default, Debug)]
pub struct ProviderMetadata {
    /// The name of provider.
    pub name: String,
}

impl ProviderMetadata {
    /// Create a new instance out of a string.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}
//
// ============================================================
//  ProviderStatus
// ============================================================

/// The status of a feature provider.
#[derive(Default, PartialEq, Eq, Debug)]
pub enum ProviderStatus {
    /// The provider has not been initialized.
    #[default]
    NotReady,

    /// The provider has been initialized, and is able to reliably resolve flag values.
    Ready,

    /// The provider is initialized but is not able to reliably resolve flag values.
    Error,

    /// The provider's cached state is no longer valid and may not be up-to-date with the source of
    /// truth.
    STALE,
}
