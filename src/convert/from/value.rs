use crate::Value;

impl From<&Value> for Value {

    fn from(v: &Value) -> Self {
        v.to_owned()
    }
}

impl From<Option<Value>> for Value {

    fn from(v: Option<Value>) -> Self {
        match v {
            Some(v) => v,
            None => Value::Null,
        }
    }
}
