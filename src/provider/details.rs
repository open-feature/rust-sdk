use crate::{EvaluationDetails, EvaluationError, EvaluationReason, FlagMetadata};

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
    pub fn new(value: T) -> Self {
        Self {
            value,
            variant: None,
            reason: None,
            error: None,
            flag_metadata: None,
        }
    }

    pub fn with_variant(mut self, variant: String) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn with_reason(mut self, reason: EvaluationReason) -> Self {
        self.reason = Some(reason);
        self
    }

    pub fn with_error(mut self, error: EvaluationError) -> Self {
        self.error = Some(error);
        self
    }

    pub fn with_flag_metadata(mut self, flag_metadata: FlagMetadata) -> Self {
        self.flag_metadata = Some(flag_metadata);
        self
    }
}
