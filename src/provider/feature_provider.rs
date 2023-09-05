use crate::{
    evaluation::{EvaluationError, FlagMetadata},
    EvaluationContext, EvaluationReason,
};

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
pub trait FeatureProvider {
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
    ///
    /// Implement [`Drop`] trait for the "shutdown" functions.
    fn initialize(&mut self, context: EvaluationContext) {}

    /// The provider MAY define a status field/accessor which indicates the readiness of the
    /// provider, with possible values NOT_READY, READY, or ERROR.
    ///
    /// Providers without this field can be assumed to be ready immediately.
    fn status(&self) -> ProviderStatus {
        ProviderStatus::default()
    }

    /// The provider interface MUST define a metadata member or accessor, containing a name field
    /// or accessor of type string, which identifies the provider implementation.
    fn metadata(&self) -> &ProviderMetadata;

    /// Resolve given [`flag_key`] as a bool value.
    fn resolve_bool_value(
        &self,
        flag_key: &str,
        default_value: bool,
        evaluation_context: Option<EvaluationContext>,
    ) -> ResolutionDetails<bool>;
}

/// The metadata of a feature provider.
#[derive(Clone, Default, Debug)]
pub struct ProviderMetadata {
    name: String,
}

impl ProviderMetadata {
    pub fn new(name: String) -> Self {
        Self { name }
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

/// A structure which contains a subset of the fields defined in the evaluation details,
/// representing the result of the provider's flag resolution process.
#[derive(Default, Debug)]
pub struct ResolutionDetails<T> {
    /// In cases of normal execution, the provider MUST populate the resolution details structure's
    /// value field with the resolved flag value.
    pub value: T,

    /// In cases of normal execution, the provider SHOULD populate the resolution details
    /// structure's variant field with a string identifier corresponding to the returned flag
    /// value.
    pub variant: Option<String>,

    /// The provider SHOULD populate the resolution details structure's reason field with "STATIC",
    /// "DEFAULT", "TARGETING_MATCH", "SPLIT", "CACHED", "DISABLED", "UNKNOWN", "ERROR" or some
    /// other string indicating the semantic reason for the returned flag value.
    pub reason: Option<EvaluationReason>,

    /// In cases of normal execution, the provider MUST NOT populate the resolution details
    /// structure's error code field, or otherwise must populate it with a null or falsy value.
    ///
    /// In cases of abnormal execution, the provider MUST indicate an error using the idioms of the
    /// implementation language, with an associated error code and optional associated error
    /// message.
    pub error: Option<EvaluationError>,

    /// The provider SHOULD populate the resolution details structure's flag metadata field.
    pub flag_metadata: Option<FlagMetadata>,
}

impl<T> ResolutionDetails<T> {
    pub fn new_successful(
        value: T,
        variant: String,
        reason: EvaluationReason,
        flag_metadata: FlagMetadata,
    ) -> Self {
        Self {
            value,
            variant: Some(variant),
            reason: Some(reason),
            flag_metadata: Some(flag_metadata),
            error: None,
        }
    }

    pub fn new_failed(value: T, error: EvaluationError) -> Self {
        Self {
            value: value,
            variant: None,
            reason: None,
            error: Some(error),
            flag_metadata: None,
        }
    }
}
