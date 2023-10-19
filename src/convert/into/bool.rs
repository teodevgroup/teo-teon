use teo_result::Error;
use crate::value::Value;

impl TryFrom<&Value> for bool {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into bool", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for bool {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into bool", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<bool> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Bool(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<bool>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<bool> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Bool(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<bool>", value.type_hint()))),
        }
    }
}

