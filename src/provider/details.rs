use derive_builder::Builder;

use crate::{EvaluationError, EvaluationReason, FlagMetadata};

/// A structure which contains a subset of the fields defined in the evaluation details,
/// representing the result of the provider's flag resolution process.
#[derive(Clone, Builder, Debug)]
#[builder(setter(strip_option))]
pub struct ResolutionDetails<T> {
    /// In cases of normal execution, the provider MUST populate the resolution details structure's
    /// value field with the resolved flag value.
    pub value: T,

    /// In cases of normal execution, the provider SHOULD populate the resolution details
    /// structure's variant field with a string identifier corresponding to the returned flag
    /// value.
    #[builder(default)]
    pub variant: Option<String>,

    /// The provider SHOULD populate the resolution details structure's reason field with "STATIC",
    /// "DEFAULT", "TARGETING_MATCH", "SPLIT", "CACHED", "DISABLED", "UNKNOWN", "ERROR" or some
    /// other string indicating the semantic reason for the returned flag value.
    #[builder(default)]
    pub reason: Option<EvaluationReason>,

    /// In cases of normal execution, the provider MUST NOT populate the resolution details
    /// structure's error code field, or otherwise must populate it with a null or falsy value.
    ///
    /// In cases of abnormal execution, the provider MUST indicate an error using the idioms of the
    /// implementation language, with an associated error code and optional associated error
    /// message.
    #[builder(default)]
    pub error: Option<EvaluationError>,

    /// The provider SHOULD populate the resolution details structure's flag metadata field.
    #[builder(default)]
    pub flag_metadata: Option<FlagMetadata>,
}

impl<T: Default> Default for ResolutionDetails<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            variant: None,
            reason: None,
            error: None,
            flag_metadata: None,
        }
    }
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

    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}
