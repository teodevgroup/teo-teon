use crate::value::Value;

impl From<usize> for Value {

    fn from(v: usize) -> Self {
        Value::Int64(v as i64)
    }
}

impl From<&usize> for Value {

    fn from(v: &usize) -> Self {
        Value::Int64(*v as i64)
    }
}

impl From<Option<usize>> for Value {

    fn from(v: Option<usize>) -> Self {
        match v {
            Some(b) => Value::Int64(b as i64),
            None => Value::Null,
        }
    }
}

impl From<Option<&usize>> for Value {

    fn from(v: Option<&usize>) -> Self {
        match v {
            Some(b) => Value::Int64(*b as i64),
            None => Value::Null,
        }
    }
}