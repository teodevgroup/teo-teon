use chrono::{DateTime, Utc};
use crate::error::Error;
use crate::value::Value;

impl TryInto<DateTime<Utc>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<DateTime<Utc>, Self::Error> {
        match self {
            Value::DateTime(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into DateTime<Utc>", self.type_hint()))),
        }
    }
}

impl TryInto<DateTime<Utc>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<DateTime<Utc>, Self::Error> {
        match self {
            Value::DateTime(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into DateTime<Utc>", self.type_hint()))),
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

impl TryInto<Option<DateTime<Utc>>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<DateTime<Utc>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::DateTime(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<DateTime<Utc>>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<DateTime<Utc>>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<DateTime<Utc>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::DateTime(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<DateTime<Utc>>", self.type_hint()))),
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
