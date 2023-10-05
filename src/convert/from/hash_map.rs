use std::collections::HashMap;
use crate::value::Value;

impl<T> From<HashMap<String, T>> for Value where T: Into<Value> {

    fn from(value: HashMap<String, T>) -> Self {
        let mut retval = HashMap::new();
        for (k, v) in value {
            retval.insert(k.to_owned(), v.into());
        }
        Value::Dictionary(retval)
    }
}

impl<T> From<Option<HashMap<String, T>>> for Value where T: Into<Value> {

    fn from(value: Option<HashMap<String, T>>) -> Self {
        if let Some(value) = value {
            let mut retval = HashMap::new();
            for (k, v) in value {
                retval.insert(k, v.into());
            }
            Value::Dictionary(retval)
        } else {
            Value::Null
        }
    }
}
