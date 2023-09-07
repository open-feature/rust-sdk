use std::{any::Any, borrow::Cow, sync::Arc};

use async_trait::async_trait;
use typed_builder::TypedBuilder;

use crate::{EvaluationContext, StructValue};

use super::ResolutionDetails;

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
    async fn initialize(&mut self, context: EvaluationContext) {}

    /// The provider MAY define a shutdown function to perform whatever cleanup is necessary for
    /// the implementation.
    async fn shutdown(&mut self) {}

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

    /// Resolve given [`flag_key`] as a bool value.
    async fn resolve_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<&EvaluationContext>,
    ) -> ResolutionDetails<bool>;

    /// Resolve given [`flag_key`] as an i64 value.
    async fn resolve_int_value(
        &self,
        flag_key: &str,
        default_value: i64,
        evaluation_context: Option<&EvaluationContext>,
    ) -> ResolutionDetails<i64>;

    /// Resolve given [`flag_key`] as a f64 value.
    async fn resolve_float_value(
        &self,
        flag_key: &str,
        default_value: f64,
        evaluation_context: Option<&EvaluationContext>,
    ) -> ResolutionDetails<f64>;

    /// Resolve given [`flag_key`] as a string value.
    async fn resolve_string_value(
        &self,
        flag_key: &str,
        default_value: &str,
        evaluation_context: Option<&EvaluationContext>,
    ) -> ResolutionDetails<String>;

    /// Resolve given [`flag_key`] as a struct value.
    async fn resolve_struct_value(
        &self,
        flag_key: &str,
        default_value: StructValue,
        evaluation_context: Option<&EvaluationContext>,
    ) -> ResolutionDetails<StructValue>;
}

/// The metadata of a feature provider.
#[derive(Clone, TypedBuilder, Default, Debug)]
pub struct ProviderMetadata {
    #[builder(setter(into))]
    name: String,
}

impl ProviderMetadata {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

/// The status of a feature provider.
#[derive(Default, Debug)]
pub enum ProviderStatus {
    #[default]
    Ready,
    NotReady,
    Error,
}
