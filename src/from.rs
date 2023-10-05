use std::collections::{BTreeMap, HashMap};
use bson::oid::ObjectId;
use chrono::{NaiveDate, DateTime, Utc};
use bigdecimal::BigDecimal;
use serde_json::Value as JsonValue;
use super::value::Value;

// MARK: - Collections

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
