use crate::value::Value;

impl From<bool> for Value {

    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<&bool> for Value {

    fn from(v: &bool) -> Self {
        Value::Bool(*v)
    }
}

impl From<Option<bool>> for Value {

    fn from(v: Option<bool>) -> Self {
        match v {
            Some(b) => Value::Bool(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&bool>> for Value {

    fn from(v: Option<&bool>) -> Self {
        match v {
            Some(b) => Value::Bool(*b),
            None => Value::Null,
        }
    }
}