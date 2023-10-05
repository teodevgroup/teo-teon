use crate::value::Value;

impl From<f64> for Value {

    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl From<&f64> for Value {

    fn from(v: &f64) -> Self {
        Value::Float(*v)
    }
}

impl From<Option<f64>> for Value {

    fn from(v: Option<f64>) -> Self {
        match v {
            Some(b) => Value::Float(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&f64>> for Value {

    fn from(v: Option<&f64>) -> Self {
        match v {
            Some(b) => Value::Float(*b),
            None => Value::Null,
        }
    }
}