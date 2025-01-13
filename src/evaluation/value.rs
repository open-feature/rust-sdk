use std::collections::HashMap;

/// Hold a value in the evaluation result of supported types.
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Struct(StructValue),
}

/// Supported types of values.
/// [spec](https://openfeature.dev/specification/types).
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum Type {
    Bool,
    Int,
    Float,
    String,
    Array,
    Struct,
}

/// Represent a structure value as defined in the
/// [spec](https://openfeature.dev/specification/types#structure).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct StructValue {
    /// The fields of struct as key-value pairs.
    pub fields: HashMap<String, Value>,
}

impl Value {
    /// Return `true` if this is a bool value.
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Try to convert `self` to bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Return `true` if this is an int value.
    pub fn is_i64(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Try to convert `self` to int.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Return `true` if this is a float value.
    pub fn is_f64(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Try to convert `self` to float.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Return `true` if this is a string value.
    pub fn is_str(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Try to convert `self` to str.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Return `true` if this is an array.
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Try to convert `self` to vector.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Return `true` if this is a struct.
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    /// Try to convert `self` to [`StructValue`].
    pub fn as_struct(&self) -> Option<&StructValue> {
        match self {
            Self::Struct(value) => Some(value),
            _ => None,
        }
    }

    /// Return the type of the value.
    pub fn get_type(&self) -> Type {
        match self {
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::String(_) => Type::String,
            Self::Array(_) => Type::Array,
            Self::Struct(_) => Type::Struct,
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
        Self::Int(value)
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
        Self::Float(value)
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
    /// Append given `key` and `value` to `self` and return it.
    #[must_use]
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.add_field(key, value);
        self
    }

    /// Append given `key` and `value` to `self` in place.
    pub fn add_field(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.fields.insert(key.into(), value.into());
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Array => write!(f, "array"),
            Self::Struct => write!(f, "struct"),
        }
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
