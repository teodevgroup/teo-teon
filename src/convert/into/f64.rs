use crate::error::Error;
use crate::value::Value;

// impl TryInto<f64> for Value {

//     type Error = Error;

//     fn try_into(self) -> Result<f64, Self::Error> {
//         match self {
//             Value::Float(b) => Ok(b),
//             _ => Err(Error::new(format!("Cannot convert {} into f64", self.type_hint()))),
//         }
//     }
// }

// impl TryInto<f64> for &Value {

//     type Error = Error;

//     fn try_into(self) -> Result<f64, Self::Error> {
//         match self {
//             Value::Float(b) => Ok(*b),
//             _ => Err(Error::new(format!("Cannot convert {} into f64", self.type_hint()))),
//         }
//     }
// }

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

// impl TryInto<Option<f64>> for Value {

//     type Error = Error;

//     fn try_into(self) -> Result<Option<f64>, Self::Error> {
//         match self {
//             Value::Null => Ok(None),
//             Value::Float(b) => Ok(Some(b)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", self.type_hint()))),
//         }
//     }
// }

// impl TryInto<Option<f64>> for &Value {

//     type Error = Error;

//     fn try_into(self) -> Result<Option<f64>, Self::Error> {
//         match self {
//             Value::Null => Ok(None),
//             Value::Float(b) => Ok(Some(*b)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", self.type_hint()))),
//         }
//     }
// }


impl TryFrom<Value> for Option<f64> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(i) => Ok(Some(i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<f64>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<f64> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(i) => Ok(Some(*i)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i32>", value.type_hint()))),
        }
    }
}
