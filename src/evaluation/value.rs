use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Struct(StructValue),
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

impl From<StructValue> for Value {
    fn from(value: StructValue) -> Self {
        Self::Struct(value)
    }
}

/// Represent a structure value as defined in the
/// [spec](https://openfeature.dev/specification/types#structure).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct StructValue {
    fields: HashMap<String, Value>,
}

impl StructValue {
    pub fn with_field<S: Into<String>, V: Into<Value>>(mut self, key: S, value: V) -> Self {
        self.add_field(key, value);
        self
    }

    pub fn add_field<S: Into<String>, V: Into<Value>>(&mut self, key: S, value: V) {
        self.fields.insert(key.into(), value.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_value() {
        let value = StructValue::default()
            .with_field("is_male", true)
            .with_field("id", 100)
            .with_field("grade", "97.5")
            .with_field("name", "Alex")
            .with_field(
                "other",
                StructValue::default().with_field("description", "A student"),
            );
    }
}
