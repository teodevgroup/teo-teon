use std::collections::{BTreeMap, HashMap};
use bson::oid::ObjectId;
use chrono::{NaiveDate, DateTime, Utc};
use bigdecimal::BigDecimal;
use serde_json::Value as JsonValue;
use super::value::Value;

// MARK: - Collections

impl<T> From<HashMap<String, T>> for Value where T: Into<Value> {
    fn from(value: HashMap<String, T>) -> Self {
        let mut retval = HashMap::new();
        for (k, v) in value {
            retval.insert(k.to_owned(), v.into());
        }
        Value::Dictionary(retval)
    }
}

impl<T> From<Value> for Vec<T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_vec().unwrap();
        let mut result: Vec<T> = vec![];
        for v in value {
            result.push(v.clone().into());
        }
        result
    }
}

impl<T> From<Value> for HashMap<String, T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: HashMap<String, T> = HashMap::new();
        for (k, v) in value {
            result.insert(k.to_owned(), (v.clone()).into());
        }
        result
    }
}

impl<T> From<Value> for BTreeMap<String, T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: BTreeMap<String, T> = BTreeMap::new();
        for (k, v) in value {
            result.insert(k.to_owned(), (v.clone()).into());
        }
        result
    }
}
