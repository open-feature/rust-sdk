use std::{any::Any, sync::Arc};

use time::OffsetDateTime;

#[derive(Clone, Debug)]
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

    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Self::Float(_) => true,
            _ => false,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(&value),
            _ => None,
        }
    }

    pub fn is_date_time(&self) -> bool {
        match self {
            Self::DateTime(_) => true,
            _ => false,
        }
    }

    pub fn as_date_time(&self) -> Option<&OffsetDateTime> {
        match self {
            Self::DateTime(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    pub fn as_struct(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        match self {
            Self::Struct(value) => Some(value.clone()),
            _ => None,
        }
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

impl<T: Any + Send + Sync> From<Arc<T>> for EvaluationContextFieldValue {
    fn from(value: Arc<T>) -> Self {
        Self::Struct(value)
    }
}

impl PartialEq for EvaluationContextFieldValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EvaluationContextFieldValue::Bool(left), EvaluationContextFieldValue::Bool(right)) => {
                left == right
            }
            (EvaluationContextFieldValue::Int(left), EvaluationContextFieldValue::Int(right)) => {
                left == right
            }
            (
                EvaluationContextFieldValue::Float(left),
                EvaluationContextFieldValue::Float(right),
            ) => left == right,
            (
                EvaluationContextFieldValue::String(left),
                EvaluationContextFieldValue::String(right),
            ) => left == right,
            (
                EvaluationContextFieldValue::DateTime(left),
                EvaluationContextFieldValue::DateTime(right),
            ) => left == right,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn evaluation_context_custom_fields() {
        let now = OffsetDateTime::now_utc();

        let context = EvaluationContext::builder()
            .targeting_key("Some Key")
            .build()
            .with_custom_field("Bool", false)
            .with_custom_field("Bool", EvaluationContextFieldValue::Bool(true))
            .with_custom_field("Int", 42)
            .with_custom_field("Float", 42.0)
            .with_custom_field("String", "StringValue")
            .with_custom_field("DateTime", now.clone())
            .with_custom_field(
                "Struct",
                EvaluationContextFieldValue::new_struct(EvaluationReason::Cached),
            );

        // Assert bool
        if let EvaluationContextFieldValue::Bool(value) = context.custom_fields.get("Bool").unwrap()
        {
            assert_eq!(true, *value);
        } else {
            panic!()
        }

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
