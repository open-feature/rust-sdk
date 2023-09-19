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
#[derive(Clone, TypedBuilder, Default, PartialEq, Debug)]
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

    pub fn merge_missing(&mut self, other: &Self) {
        if self.targeting_key.is_none() {
            if let Some(targeting_key) = &other.targeting_key {
                self.targeting_key = Some(targeting_key.clone());
            }
        }

        other.custom_fields.iter().for_each(|(key, value)| {
            if !self.custom_fields.contains_key(key) {
                self.custom_fields.insert(key.clone(), value.clone());
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_missig_given_empty() {
        let mut context = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build()
            .with_custom_field("Some", "Value");

        let expected = context.clone();

        context.merge_missing(&EvaluationContext::default());

        assert_eq!(context, expected);
    }

    #[test]
    fn merge_missing_given_targeting_key() {
        let mut context = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build();

        let expected = context.clone();

        context.merge_missing(
            &EvaluationContext::builder()
                .targeting_key("Another Key")
                .build(),
        );

        assert_eq!(context, expected);
    }

    #[test]
    fn merge_missing_given_custom_fields() {
        let mut context = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build()
            .with_custom_field("Key", "Value");

        context.merge_missing(
            &EvaluationContext::default()
                .with_custom_field("Key", "Another Value")
                .with_custom_field("Another Key", "Value"),
        );

        assert_eq!(
            context,
            EvaluationContext::builder()
                .targeting_key("Targeting Key")
                .build()
                .with_custom_field("Key", "Value")
                .with_custom_field("Another Key", "Value")
        )
    }

    #[test]
    fn merge_missing_given_full() {
        let mut context = EvaluationContext::default();

        let other = EvaluationContext::builder()
            .targeting_key("Targeting Key")
            .build()
            .with_custom_field("Key", "Value");

        context.merge_missing(&other);

        assert_eq!(context, other);
    }
}

