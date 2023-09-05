use std::{any::Any, sync::Arc};

use time::OffsetDateTime;

pub enum EvaluationContextFieldValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    DateTime(OffsetDateTime),
    Struct(Arc<dyn Any + Send + Sync>),
}

impl EvaluationContextFieldValue {
    pub fn new_struct<T>(value: T) -> Self
    where
        T: Any + Send + Sync,
    {
        Self::Struct(Arc::new(value))
    }
}

impl From<bool> for EvaluationContextFieldValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for EvaluationContextFieldValue {
    fn from(value: i8) -> Self {
        Self::Int(value.into())
    }
}

impl From<i16> for EvaluationContextFieldValue {
    fn from(value: i16) -> Self {
        Self::Int(value.into())
    }
}

impl From<i32> for EvaluationContextFieldValue {
    fn from(value: i32) -> Self {
        Self::Int(value.into())
    }
}

impl From<i64> for EvaluationContextFieldValue {
    fn from(value: i64) -> Self {
        Self::Int(value.into())
    }
}

impl From<u8> for EvaluationContextFieldValue {
    fn from(value: u8) -> Self {
        Self::Int(value.into())
    }
}

impl From<u16> for EvaluationContextFieldValue {
    fn from(value: u16) -> Self {
        Self::Int(value.into())
    }
}

impl From<u32> for EvaluationContextFieldValue {
    fn from(value: u32) -> Self {
        Self::Int(value.into())
    }
}

impl From<f32> for EvaluationContextFieldValue {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<f64> for EvaluationContextFieldValue {
    fn from(value: f64) -> Self {
        Self::Float(value.into())
    }
}

impl From<String> for EvaluationContextFieldValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for EvaluationContextFieldValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<OffsetDateTime> for EvaluationContextFieldValue {
    fn from(value: OffsetDateTime) -> Self {
        Self::DateTime(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn evaluation_context_custom_fields() {
        let now = OffsetDateTime::now_utc();

        let context = EvaluationContext::new()
            .with_targeting_key("Some Key".to_string())
            .with_custom_field("Bool".to_string(), true.into())
            .with_custom_field("Int".to_string(), 42.into())
            .with_custom_field("Float".to_string(), 42.0.into())
            .with_custom_field("String".to_string(), "StringValue".into())
            .with_custom_field("DateTime".to_string(), now.clone().into())
            .with_custom_field(
                "Struct".to_string(),
                EvaluationContextFieldValue::new_struct(EvaluationReason::Cached),
            );

        // Assert bool
        if let EvaluationContextFieldValue::Bool(value) = context.custom_fields.get("Bool").unwrap()
        {
            assert_eq!(true, *value);
        } else {
            panic!()
        };

        // Assert int.
        if let EvaluationContextFieldValue::Int(value) = context.custom_fields.get("Int").unwrap() {
            assert_eq!(42, *value);
        } else {
            panic!()
        }

        // Assert float.
        if let EvaluationContextFieldValue::Float(value) =
            context.custom_fields.get("Float").unwrap()
        {
            assert_eq!(42.0, *value);
        } else {
            panic!()
        }

        // Assert string.
        if let EvaluationContextFieldValue::String(value) =
            context.custom_fields.get("String").unwrap()
        {
            assert_eq!("StringValue", value);
        } else {
            panic!()
        }

        // Assert date time.
        if let EvaluationContextFieldValue::DateTime(value) =
            context.custom_fields.get("DateTime").unwrap()
        {
            assert_eq!(now, *value);
        } else {
            panic!()
        }

        // Assert struct.
        if let EvaluationContextFieldValue::Struct(value) =
            context.custom_fields.get("Struct").unwrap().clone()
        {
            let v = value.clone().downcast::<EvaluationReason>().unwrap();
            assert_eq!(EvaluationReason::Cached, *v);
        } else {
            panic!()
        };
    }
}
