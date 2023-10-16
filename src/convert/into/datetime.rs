use chrono::{DateTime, Utc};
use crate::error::Error;
use crate::value::Value;

impl TryFrom<&Value> for DateTime<Utc> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(d) => Ok(*d),
            _ => Err(Error::new(format!("Cannot convert {} into DateTime<Utc>", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for DateTime<Utc> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(d) => Ok(d),
            _ => Err(Error::new(format!("Cannot convert {} into DateTime<Utc>", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<DateTime<Utc>> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::DateTime(d) => Ok(Some(d)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<DateTime<Utc>>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<DateTime<Utc>> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::DateTime(d) => Ok(Some(*d)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<DateTime<Utc>>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a DateTime<Utc> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(d) => Ok(d),
            _ => Err(Error::new(format!("Cannot convert {} into &DateTime<Utc>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a DateTime<Utc>> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::DateTime(d) => Ok(Some(d)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&DateTime<Utc>>", value.type_hint()))),
        }
    }
}
