use crate::value::Value;

impl From<&String> for Value {

    fn from(v: &String) -> Self {
        Self::from(v.clone())
    }
}

impl From<String> for Value {

    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<Option<String>> for Value {

    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Value::String(s),
            None => Value::Null,
        }
    }
}

impl From<Option<&String>> for Value {

    fn from(value: Option<&String>) -> Self {
        Self::from(value.cloned())
    }
}