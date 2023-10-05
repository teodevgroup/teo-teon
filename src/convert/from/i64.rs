use crate::value::Value;

impl From<i64> for Value {

    fn from(v: i64) -> Self {
        Value::Int64(v)
    }
}

impl From<&i64> for Value {

    fn from(v: &i64) -> Self {
        Value::Int64(*v)
    }
}

impl From<Option<i64>> for Value {

    fn from(v: Option<i64>) -> Self {
        match v {
            Some(b) => Value::Int64(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&i64>> for Value {

    fn from(v: Option<&i64>) -> Self {
        match v {
            Some(b) => Value::Int64(*b),
            None => Value::Null,
        }
    }
}