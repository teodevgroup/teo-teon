use crate::value::Value;

impl From<&str> for Value {

    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<Option<&str>> for Value {

    fn from(v: Option<&str>) -> Self {
        match v {
            Some(s) => Self::from(s),
            None => Value::Null,
        }
    }
}