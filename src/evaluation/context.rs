use std::{any::Any, collections::HashMap, sync::Arc};

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

pub enum EvaluationContextFieldValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Datetime,
    Struct(Arc<dyn Any + Send + Sync + 'static>),
}

pub trait StructValue: Send + Sync + 'static {}

impl<T: Any> StructValue for T where T: Any + Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{EvaluationDetails, EvaluationReason};

    use super::*;

    #[test]
    fn evaluation_context_struct_field_value() {
        let mut context = EvaluationContext::new()
            .with_targeting_key("Some Key".to_string())
            .with_custom_field(
                "Key".to_string(),
                EvaluationContextFieldValue::Struct(Arc::new(EvaluationReason::Cached)),
            );

        context.custom_fields.insert(
            "Key".to_string(),
            EvaluationContextFieldValue::Struct(Arc::new(EvaluationReason::Cached)),
        );

        if let EvaluationContextFieldValue::Struct(value) =
            context.custom_fields.get("Key").unwrap().clone()
        {
            let v = value.clone().downcast::<EvaluationReason>().unwrap();
            assert_eq!(EvaluationReason::Cached, *v);
        }
    }
}
