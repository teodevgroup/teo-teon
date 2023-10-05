use std::collections::BTreeMap;
use crate::value::Value;

impl<T> From<BTreeMap<String, T>> for Value where T: Into<Value> {

    fn from(value: BTreeMap<String, T>) -> Self {
        let mut retval = BTreeMap::new();
        for (k, v) in value {
            retval.insert(k.to_owned(), v.into());
        }
        Value::BTreeDictionary(retval)
    }
}

impl<T> From<Option<BTreeMap<String, T>>> for Value where T: Into<Value> {

    fn from(value: Option<BTreeMap<String, T>>) -> Self {
        if let Some(value) = value {
            let mut retval = BTreeMap::new();
            for (k, v) in value {
                retval.insert(k, v.into());
            }
            Value::BTreeDictionary(retval)
        } else {
            Value::Null
        }
    }
}
