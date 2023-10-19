use teo_result::Error;
use crate::value::Value;

impl TryFrom<Value> for f32 {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into f32", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for f32 {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float32(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into f32", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<f32> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Float32(f) => Ok(Some(f)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f32>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<f32> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Float32(f) => Ok(Some(*f)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f32>", value.type_hint()))),
        }
    }
}

