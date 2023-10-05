use indexmap::IndexMap;
use crate::value::Value;

impl<T> From<IndexMap<String, T>> for Value where T: Into<Value> {

    fn from(value: IndexMap<String, T>) -> Self {
        let mut retval = IndexMap::new();
        for (k, v) in value {
            retval.insert(k.to_owned(), v.into());
        }
        Value::IndexDictionary(retval)
    }
}

impl<T> From<Option<IndexMap<String, T>>> for Value where T: Into<Value> {

    fn from(value: Option<IndexMap<String, T>>) -> Self {
        if let Some(value) = value {
            let mut retval = IndexMap::new();
            for (k, v) in value {
                retval.insert(k, v.into());
            }
            Value::IndexDictionary(retval)
        } else {
            Value::Null
        }
    }
}
