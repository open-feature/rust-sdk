use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct EvaluationDetails<T> {
    pub flag_key: String,
    pub value: T,
    pub reason: Option<EvaluationReason>,
    pub error: Option<EvaluationError>,
    pub variant: Option<String>,
    pub flag_metadata: FlagMetadata,
}

/// Reason for evaluation.
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub enum EvaluationReason {
    /// The resolved value is static (no dynamic evaluation).
    Static,

    /// The resolved value fell back to a pre-configured value (no dynamic evaluation occurred or
    /// dynamic evaluation yielded no result).
    Default,

    /// The resolved value was the result of a dynamic evaluation, such as a rule or specific
    /// user-targeting.
    TargetingMatch,

    /// The resolved value was the result of pseudorandom assignment.
    Split,

    /// The resolved value was retrieved from cache.
    Cached,

    /// The resolved value was the result of the flag being disabled in the management system.
    Disabled,

    #[default]
    /// The reason for the resolved value could not be determined.
    Unknown,

    /// The resolved value was the result of an error.
    Error,

    /// Other custom reason.
    Other(String),
}

impl ToString for EvaluationReason {
    fn to_string(&self) -> String {
        match self {
            Self::Static => "STATIC",
            Self::Default => "DEFAULT",
            Self::TargetingMatch => "TARGETING_MATCH",
            Self::Split => "SPLIT",
            Self::Cached => "CACHED",
            Self::Disabled => "DISABLED",
            Self::Unknown => "UNKNOWN",
            Self::Error => "ERROR",
            Self::Other(reason) => reason.as_str(),
        }
        .to_string()
    }
}

/// Struct representing error
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EvaluationError {
    pub code: EvaluationErrorCode,
    pub message: Option<String>,
}

/// An enumerated error code represented idiomatically in the implementation language.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EvaluationErrorCode {
    /// The value was resolved before the provider was initialized.
    ProviderNotReady,

    /// The flag could not be found.
    FlagNotFound,

    /// An error was encountered parsing data, such as a flag configuration.
    ParseError,

    /// The type of the flag value does not match the expected type.
    TypeMismatch,

    /// The provider requires a targeting key and one was not provided in the evaluation context.
    TargetingKeyMissing,

    /// The evaluation context does not meet provider requirements.
    InvalidContext,

    /// The error was for a reason not enumerated above.
    General(String),
}

/// A structure which supports definition of arbitrary properties, with keys of type string, and
/// values of type boolean, string, or number.
///
/// This structure is populated by a provider for use by an Application Author (via the Evaluation
/// API) or an Application Integrator (via hooks).
#[derive(Clone, Default, Debug)]
pub struct FlagMetadata {
    pub values: HashMap<String, FlagMetadataValue>,
}

#[derive(Clone, Debug)]
pub enum FlagMetadataValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}
