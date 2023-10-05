use crate::error::Error;
use crate::value::Value;

impl TryInto<f64> for Value {

    type Error = Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Float(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into f64", self.type_hint()))),
        }
    }
}

impl TryInto<f64> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Float(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into f64", self.type_hint()))),
        }
    }
}

impl TryInto<Option<f64>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<f64>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Float(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<f64>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<f64>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Float(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", self.type_hint()))),
        }
    }
}
