use chrono::{DateTime, Utc};
use crate::value::Value;

impl From<DateTime<Utc>> for Value {

    fn from(v: DateTime<Utc>) -> Self {
        Value::DateTime(v)
    }
}

impl From<&DateTime<Utc>> for Value {

    fn from(v: &DateTime<Utc>) -> Self {
        Value::DateTime(*v)
    }
}

impl From<Option<DateTime<Utc>>> for Value {

    fn from(v: Option<DateTime<Utc>>) -> Self {
        match v {
            Some(b) => Value::DateTime(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&DateTime<Utc>>> for Value {

    fn from(v: Option<&DateTime<Utc>>) -> Self {
        match v {
            Some(b) => Value::DateTime(*b),
            None => Value::Null,
        }
    }
}