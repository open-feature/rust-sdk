use std::collections::HashMap;

use typed_builder::TypedBuilder;

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
#[derive(Clone, TypedBuilder, Default)]
pub struct EvaluationContext {
    /// The targeting key uniquely identifies the subject (end-user, or client service) of a flag
    /// evaluation. Providers may require this field for fractional flag evaluation, rules, or
    /// overrides targeting specific users. Such providers may behave unpredictably if a targeting
    /// key is not specified at flag resolution.
    #[builder(default, setter(into, strip_option))]
    pub targeting_key: Option<String>,

    #[builder(default)]
    pub custom_fields: HashMap<String, EvaluationContextFieldValue>,
}

impl EvaluationContext {
    pub fn with_custom_field<S: Into<String>, V: Into<EvaluationContextFieldValue>>(
        mut self,
        key: S,
        value: V,
    ) -> Self {
        self.add_custom_field(key, value);
        self
    }

    pub fn add_custom_field<S: Into<String>, V: Into<EvaluationContextFieldValue>>(
        &mut self,
        key: S,
        value: V,
    ) {
        self.custom_fields.insert(key.into(), value.into());
    }
}
