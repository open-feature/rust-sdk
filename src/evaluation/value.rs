use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Struct(StructValue),
}

/// Represent a structure value as defined in the
/// [spec](https://openfeature.dev/specification/types#structure).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct StructValue {
    pub fields: HashMap<String, Value>,
}

impl Value {
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

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    pub fn as_struct(&self) -> Option<&StructValue> {
        match self {
            Self::Struct(value) => Some(value),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Int(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Int(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value.into())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Int(value.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::Int(value.into())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Int(value.into())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Vec<T>) -> Self {
        Self::Array(value.into_iter().map(Into::into).collect())
    }
}

impl From<StructValue> for Value {
    fn from(value: StructValue) -> Self {
        Self::Struct(value)
    }
}

impl StructValue {
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.add_field(key, value);
        self
    }

    pub fn add_field(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.fields.insert(key.into(), value.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_value() {
        let alex = StructValue::default()
            .with_field("is_male", false)
            .with_field("id", 100)
            .with_field("grade", 97.5)
            .with_field("name", "Alex")
            .with_field("friends", vec!["Bob", "Carl"])
            .with_field(
                "other",
                StructValue::default().with_field("description", "A student"),
            );

        let is_male = alex.fields.get("is_male").unwrap();
        assert!(is_male.is_bool());
        assert_eq!(false, is_male.as_bool().unwrap());

        let id = alex.fields.get("id").unwrap();
        assert!(id.is_i64());
        assert_eq!(100, id.as_i64().unwrap());

        let grade = alex.fields.get("grade").unwrap();
        assert!(grade.is_f64());
        assert_eq!(97.5, grade.as_f64().unwrap());

        let name = alex.fields.get("name").unwrap();
        assert!(name.is_str());
        assert_eq!("Alex", alex.fields.get("name").unwrap().as_str().unwrap());

        let friends = alex.fields.get("friends").unwrap();
        assert!(friends.is_array());
        assert_eq!(
            vec![
                Value::String("Bob".to_string()),
                Value::String("Carl".to_string())
            ],
            *friends.as_array().unwrap()
        );

        let other = alex.fields.get("other").unwrap();
        assert!(other.is_struct());
        assert_eq!(
            "A student",
            other
                .as_struct()
                .unwrap()
                .fields
                .get("description")
                .unwrap()
                .as_str()
                .unwrap()
        );
    }
}
