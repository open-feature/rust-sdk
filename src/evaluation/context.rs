use std::{any::Any, collections::HashMap};

/// The evaluation context provides ambient information for the purposes of flag evaluation.
/// Contextual data may be used as the basis for targeting, including rule-based evaluation,
/// overrides for specific subjects, or fractional flag evaluation.
///
/// The context might contain information about the end-user, the application, the host, or any
/// other ambient data that might be useful in flag evaluation. For example, a flag system might
/// define rules that return a specific value based on the user's email address, locale, or the
/// time of day. The context provides this information. The context can be optionally provided at
/// evaluation, and mutated in before hooks.
pub struct EvaluationContext {
    /// The targeting key uniquely identifies the subject (end-user, or client service) of a flag
    /// evaluation. Providers may require this field for fractional flag evaluation, rules, or
    /// overrides targeting specific users. Such providers may behave unpredictably if a targeting
    /// key is not specified at flag resolution.
    targeting_key: Option<String>,

    custom_fields: HashMap<String, Box<dyn StructValue>>,
}

pub enum EvaluationContextFieldValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Datetime,
    Struct(Box<dyn StructValue>),
}

pub trait StructValue {}

impl EvaluationContext {
    pub fn keys() {}
}

impl<T: Any> StructValue for T {}
