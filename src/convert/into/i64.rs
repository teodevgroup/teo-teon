use teo_result::Error;
use crate::value::Value;

impl TryFrom<&Value> for i64 {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into i64", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for i64 {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into i64", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<i64> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int64(i) => Ok(Some(i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i64>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<i64> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int64(i) => Ok(Some(*i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i64>", value.type_hint()))),
        }
    }
}
