use teo_result::Error;
use crate::value::Value;

impl TryFrom<&Value> for f64 {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into f64", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for f64 {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into f64", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<f64> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Float(f) => Ok(Some(f)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<f64> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Float(f) => Ok(Some(*f)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a f64 {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into &f64", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a f64> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Float(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&f64>", value.type_hint()))),
        }
    }
}

