use std::collections::HashMap;

use crate::EvaluationError;

pub type EvaluationResult<T> = Result<T, EvaluationError>;

// ============================================================
//  EvaluationDetails
// ============================================================

#[derive(Clone, Default, Debug)]
pub struct EvaluationDetails<T> {
    pub flag_key: String,
    pub value: T,
    pub reason: Option<EvaluationReason>,
    pub variant: Option<String>,
    pub flag_metadata: FlagMetadata,
}

// ============================================================
//  EvaluationReason
// ============================================================

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

    /// The reason for the resolved value could not be determined.
    #[default]
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

// ============================================================
//  FlagMetadata
// ============================================================

/// A structure which supports definition of arbitrary properties, with keys of type string, and
/// values of type boolean, string, or number.
///
/// This structure is populated by a provider for use by an Application Author (via the Evaluation
/// API) or an Application Integrator (via hooks).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct FlagMetadata {
    pub values: HashMap<String, FlagMetadataValue>,
}

impl FlagMetadata {
    pub fn with_value(
        mut self,
        key: impl Into<String>,
        value: impl Into<FlagMetadataValue>,
    ) -> Self {
        self.add_value(key, value);
        self
    }

    pub fn add_value(&mut self, key: impl Into<String>, value: impl Into<FlagMetadataValue>) {
        self.values.insert(key.into(), value.into());
    }
}

// ============================================================
//  FlagMetadataValue
// ============================================================

#[derive(Clone, PartialEq, Debug)]
pub enum FlagMetadataValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl From<bool> for FlagMetadataValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for FlagMetadataValue {
    fn from(value: i8) -> Self {
        Self::Int(value.into())
    }
}

impl From<i16> for FlagMetadataValue {
    fn from(value: i16) -> Self {
        Self::Int(value.into())
    }
}

impl From<i32> for FlagMetadataValue {
    fn from(value: i32) -> Self {
        Self::Int(value.into())
    }
}

impl From<i64> for FlagMetadataValue {
    fn from(value: i64) -> Self {
        Self::Int(value.into())
    }
}

impl From<u8> for FlagMetadataValue {
    fn from(value: u8) -> Self {
        Self::Int(value.into())
    }
}

impl From<u16> for FlagMetadataValue {
    fn from(value: u16) -> Self {
        Self::Int(value.into())
    }
}

impl From<u32> for FlagMetadataValue {
    fn from(value: u32) -> Self {
        Self::Int(value.into())
    }
}

impl From<f32> for FlagMetadataValue {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<f64> for FlagMetadataValue {
    fn from(value: f64) -> Self {
        Self::Float(value.into())
    }
}

impl From<String> for FlagMetadataValue {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<&str> for FlagMetadataValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
