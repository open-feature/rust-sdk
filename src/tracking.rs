//! Types related to tracking.
//!
//! See the [spec](https://openfeature.dev/specification/sections/tracking).

use std::collections::HashMap;

use typed_builder::TypedBuilder;

use crate::Value;

// ============================================================
//  TrackingEventDetails
// ============================================================

/// The details of a tracking event, passed to [`crate::Client::track`] and forwarded to the
/// provider's [`crate::provider::FeatureProvider::track`].
///
/// See the [spec](https://openfeature.dev/specification/sections/tracking#62-tracking-event-details).
#[derive(Clone, Default, Debug, TypedBuilder)]
pub struct TrackingEventDetails {
    /// An optional scalar amount associated with the tracking event, e.g. the monetary value of a
    /// purchase (spec 6.2.1).
    #[builder(default, setter(strip_option))]
    pub value: Option<f64>,

    /// Arbitrary custom fields describing the tracking event. Keys are strings and values are any
    /// of the supported [`Value`] types (spec 6.2.2).
    #[builder(default)]
    pub values: HashMap<String, Value>,
}

impl TrackingEventDetails {
    /// Insert a custom field into [`TrackingEventDetails::values`], returning `self` for chaining.
    #[must_use]
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use spec::spec;

    use super::*;
    use crate::StructValue;

    #[spec(
        number = "6.2.1",
        text = "The tracking event details structure MUST define an optional numeric value, associating a scalar quality with an tracking event."
    )]
    #[test]
    fn value_is_optional_and_numeric() {
        let without = TrackingEventDetails::default();
        assert_eq!(without.value, None);

        let with = TrackingEventDetails::builder().value(99.9).build();
        assert_eq!(with.value, Some(99.9));
    }

    #[spec(
        number = "6.2.2",
        text = "The tracking event details MUST support the inclusion of custom fields, having keys of type string, and values of type boolean | string | number | structure."
    )]
    #[test]
    fn supports_custom_fields_of_every_value_type() {
        let mut structure = StructValue::default();
        structure
            .fields
            .insert("nested".to_string(), Value::Bool(true));

        let details = TrackingEventDetails::default()
            .with_field("bool", true)
            .with_field("string", "hello".to_string())
            .with_field("int", 42i64)
            .with_field("float", 3.5f64)
            .with_field("struct", Value::Struct(structure.clone()));

        assert_eq!(details.values.get("bool"), Some(&Value::Bool(true)));
        assert_eq!(
            details.values.get("string"),
            Some(&Value::String("hello".to_string()))
        );
        assert_eq!(details.values.get("int"), Some(&Value::Int(42)));
        assert_eq!(details.values.get("float"), Some(&Value::Float(3.5)));
        assert_eq!(
            details.values.get("struct"),
            Some(&Value::Struct(structure))
        );
    }
}
