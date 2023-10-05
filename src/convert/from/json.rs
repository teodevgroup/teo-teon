use serde_json::{Value as JsonValue};
use crate::value::Value;

impl From<&JsonValue> for Value {

    fn from(value: &JsonValue) -> Self {
        match value {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(b) => Self::Bool(*b),
            JsonValue::Number(n) => if n.is_i64() {
                Self::Int64(n.as_i64().unwrap())
            } else if n.is_f64() {
                Self::Float(n.as_f64().unwrap())
            } else if n.is_u64() {
                Self::Int64(n.as_i64().unwrap())
            } else {
                unreachable!()
            },
            JsonValue::String(s) => Self::String(s.clone()),
            JsonValue::Array(vec) => Self::Array(vec.iter().map(|v| {
                Self::from(v)
            }).collect()),
            JsonValue::Object(obj) => Self::Dictionary(obj.iter().map(|(k, v)| {
                (k.to_owned(), Self::from(v))
            }).collect()),
        }
    }
}

impl From<JsonValue> for Value {

    fn from(value: JsonValue) -> Self {
        Self::from(&value)
    }
}