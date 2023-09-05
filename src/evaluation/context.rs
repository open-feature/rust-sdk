use std::collections::HashMap;

use crate::EvaluationContextFieldValue;

/// The evaluation context provides ambient information for the purposes of flag evaluation.
/// Contextual data may be used as the basis for targeting, including rule-based evaluation,
/// overrides for specific subjects, or fractional flag evaluation.
///
/// The context might contain information about the end-user, the application, the host, or any
/// other ambient data that might be useful in flag evaluation. For example, a flag system might
/// define rules that return a specific value based on the user's email address, locale, or the
/// time of day. The context provides this information. The context can be optionally provided at
/// evaluation, and mutated in before hooks.
#[derive(Default)]
pub struct EvaluationContext {
    /// The targeting key uniquely identifies the subject (end-user, or client service) of a flag
    /// evaluation. Providers may require this field for fractional flag evaluation, rules, or
    /// overrides targeting specific users. Such providers may behave unpredictably if a targeting
    /// key is not specified at flag resolution.
    pub targeting_key: Option<String>,

    pub custom_fields: HashMap<String, EvaluationContextFieldValue>,
}

impl EvaluationContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_targeting_key(mut self, targeting_key: String) -> Self {
        self.targeting_key = Some(targeting_key);
        self
    }

    pub fn with_custom_fields(
        mut self,
        custom_fields: HashMap<String, EvaluationContextFieldValue>,
    ) -> Self {
        self.custom_fields = custom_fields;
        self
    }

    pub fn with_custom_field(mut self, key: String, value: EvaluationContextFieldValue) -> Self {
        self.custom_fields.insert(key, value);
        self
    }
}
