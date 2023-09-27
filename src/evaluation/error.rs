// ============================================================
//  EvaluationError
// ============================================================

use typed_builder::TypedBuilder;

/// Struct representing error
#[derive(Clone, Eq, PartialEq, TypedBuilder, Debug)]
pub struct EvaluationError {
    /// The error code of abnormal evaluation.
    pub code: EvaluationErrorCode,

    /// The custom error message returned by the provider.
    #[builder(default, setter(strip_option, into))]
    pub message: Option<String>,
}

// ============================================================
//  EvaluationErrorCode
// ============================================================

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

impl ToString for EvaluationErrorCode {
    fn to_string(&self) -> String {
        match self {
            Self::ProviderNotReady => "PROVIDER_NOT_READY".to_string(),
            Self::FlagNotFound => "FLAG_NOT_FOUND".to_string(),
            Self::ParseError => "PARSE_ERROR".to_string(),
            Self::TypeMismatch => "TYPE_MISMATCH".to_string(),
            Self::TargetingKeyMissing => "TARGETING_KEY_MISSING".to_string(),
            Self::InvalidContext => "INVALID_CONTEXT".to_string(),
            Self::General(message) => message.clone(),
        }
    }
}
