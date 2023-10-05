use crate::error::Error;
use crate::value::Value;

impl TryInto<i64> for Value {

    type Error = Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int64(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into i64", self.type_hint()))),
        }
    }
}

impl TryInto<i64> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int64(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into i64", self.type_hint()))),
        }
    }
}

impl TryInto<Option<i64>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<i64>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Int64(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i64>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<i64>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<i64>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Int64(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i64>", self.type_hint()))),
        }
    }
}
