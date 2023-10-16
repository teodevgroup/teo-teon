use crate::error::Error;
use crate::value::Value;

impl TryFrom<&Value> for usize {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(u) => Ok(*u as usize),
            Value::Int64(u) => Ok(*u as usize),
            _ => Err(Error::new(format!("Cannot convert {} into usize", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for usize {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(u) => Ok(u as usize),
            Value::Int64(u) => Ok(u as usize),
            _ => Err(Error::new(format!("Cannot convert {} into usize", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<usize> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int(u) => Ok(Some(u as usize)),
            Value::Int64(u) => Ok(Some(u as usize)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<usize>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<usize> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int(u) => Ok(Some(*u as usize)),
            Value::Int64(u) => Ok(Some(*u as usize)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<usize>", value.type_hint()))),
        }
    }
}

