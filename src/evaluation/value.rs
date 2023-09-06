use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Struct(StructValue),
}

impl Value {
    //pub fn as_bool(&self) -> Option<bool> {
    //    if let Self::Bool(value) = self {
    //        Some(*value)
    //    } else {
    //        None
    //    }
    //}

    //pub fn as_int(&self) -> Option<i64> {
    //    if let Self::Int(value) = self {
    //        Some(*value)
    //    } else {
    //        None
    //    }
    //}

    //pub fn as_float(&self) -> Option<f64> {
    //    if let Self::Float(value) = self {
    //        Some(*value)
    //    } else {
    //        None
    //    }
    //}

    //pub fn as_string(&self) -> Option<String> {
    //    if let Self::String(value) = self {
    //        Some(value.clone())
    //    } else {
    //        None
    //    }
    //}

    //pub fn as_struct<T: From<StructValue>>(&self) -> Option<T> {
    //    if let Self::Struct(value) = self {
    //        value.into()
    //    } else {
    //        None
    //    }
    //}
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

/// Represent a structure value as defined in the
/// [spec](https://openfeature.dev/specification/types#structure).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct StructValue {
    fields: HashMap<String, Value>,
}

impl StructValue {
    pub fn with_field(mut self, key: String, value: Value) -> Self {
        self.add_field(key, value);
        self
    }

    pub fn add_field(&mut self, key: String, value: Value) {
        self.fields.insert(key, value);
    }
}
