// ============================================================
//  EvaluationError
// ============================================================

/// Struct representing error
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EvaluationError {
    pub code: EvaluationErrorCode,
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
