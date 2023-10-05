use crate::error::Error;
use crate::value::Value;

impl TryInto<bool> for Value {

    type Error = Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into bool", self.type_hint()))),
        }
    }
}

impl TryInto<bool> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into bool", self.type_hint()))),
        }
    }
}

impl TryInto<Option<bool>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<bool>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Bool(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<bool>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<bool>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<bool>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Bool(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<bool>", self.type_hint()))),
        }
    }
}
