use crate::error::Error;
use crate::value::Value;

impl TryFrom<&Value> for i32 {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into i32", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for i32 {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into i32", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<i32> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int(i) => Ok(Some(i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i32>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<i32> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int(i) => Ok(Some(*i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i32>", value.type_hint()))),
        }
    }
}
