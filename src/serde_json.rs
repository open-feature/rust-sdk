use crate::{EvaluationError, EvaluationResult, StructValue, Value};

impl TryFrom<serde_json::Value> for Value {
    type Error = EvaluationError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        json_value_to_value(&value)
    }
}

impl TryFrom<&serde_json::Value> for Value {
    type Error = EvaluationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        json_value_to_value(value)
    }
}

fn json_value_to_value(value: &serde_json::Value) -> EvaluationResult<Value> {
    match value {
        serde_json::Value::Bool(value) => Ok(Value::Bool(*value)),
        serde_json::Value::Number(value) if value.is_i64() => {
            Ok(Value::Int(value.as_i64().unwrap()))
        }
        serde_json::Value::Number(value) if value.is_f64() => {
            Ok(Value::Float(value.as_f64().unwrap()))
        }
        serde_json::Value::String(value) => Ok(Value::String(value.to_string())),
        serde_json::Value::Array(array) => Ok(Value::Array(
            array
                .iter()
                .map(|x| json_value_to_value(x))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        serde_json::Value::Object(object) => {
            let mut result = StructValue::default();

            for (key, value) in object {
                result.add_field(key, json_value_to_value(value)?);
            }

            Ok(Value::Struct(result))
        }
        _ => Err(EvaluationError::builder()
            .code(crate::EvaluationErrorCode::TypeMismatch)
            .message("Failed to convert from JSON")
            .build()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{StructValue, Value};

    #[test]
    fn convert_data() {
        let json = serde_json::json!({
            "id": 100,
            "name": "Bob",
            "age": 23.5,
            "active": true,
            "phones": [
                "12345",
                "67890",
                [123, 456]
            ],
            "children": [{
                "name": "Carl",
                "gender": "male"
            }, {
                "name": "Dina",
                "gender": "female",
            }]
        });

        let expected_value = Value::Struct(
            StructValue::default()
                .with_field("id", 100)
                .with_field("name", "Bob")
                .with_field("age", 23.5)
                .with_field("active", true)
                .with_field(
                    "phones",
                    Value::Array(vec![
                        "12345".into(),
                        "67890".into(),
                        Value::Array(vec![123.into(), 456.into()]),
                    ]),
                )
                .with_field(
                    "children",
                    Value::Array(vec![
                        Value::Struct(
                            StructValue::default()
                                .with_field("name", "Carl")
                                .with_field("gender", "male"),
                        ),
                        Value::Struct(
                            StructValue::default()
                                .with_field("name", "Dina")
                                .with_field("gender", "female"),
                        ),
                    ]),
                ),
        );

        assert_eq!(expected_value, Value::try_from(&json).unwrap());
        assert_eq!(expected_value, Value::try_from(json).unwrap());
    }

    #[test]
    fn convert_invalid_data() {
        let json = serde_json::Value::Null;
        let result = Value::try_from(json);

        assert!(result.is_err());
    }
}
