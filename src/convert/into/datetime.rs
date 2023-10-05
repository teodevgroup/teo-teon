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
