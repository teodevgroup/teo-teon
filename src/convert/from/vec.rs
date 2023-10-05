use crate::value::Value;

impl<T> From<Vec<T>> for Value where T: Into<Value> {

    fn from(value: Vec<T>) -> Self {
        let mut retval = Vec::new();
        for v in value {
            retval.push(v.into());
        }
        Value::Array(retval)
    }
}

impl<T> From<Option<Vec<T>>> for Value where T: Into<Value> {

    fn from(value: Option<Vec<T>>) -> Self {
        if let Some(value) = value {
            let mut retval = Vec::new();
            for v in value {
                retval.push(v.into());
            }
            Value::Array(retval)
        } else {
            Value::Null
        }
    }
}
