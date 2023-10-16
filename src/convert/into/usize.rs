use crate::error::Error;
use crate::value::Value;

// impl TryInto<usize> for Value {

//     type Error = Error;

//     fn try_into(self) -> Result<usize, Self::Error> {
//         match self {
//             Value::Int(b) => Ok(b as usize),
//             Value::Int64(b) => Ok(b as usize),
//             _ => Err(Error::new(format!("Cannot convert {} into usize", self.type_hint()))),
//         }
//     }
// }

impl TryFrom<&Value> for usize {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(b) => Ok(*b as usize),
            Value::Int64(b) => Ok(*b as usize),
            _ => Err(Error::new(format!("Cannot convert {} into usize", value.type_hint()))),
        }
    }
}

// impl TryInto<usize> for &Value {

//     type Error = Error;

//     fn try_into(self) -> Result<usize, Self::Error> {
//         match self {
//             Value::Int(b) => Ok(*b as usize),
//             Value::Int64(b) => Ok(*b as usize),
//             _ => Err(Error::new(format!("Cannot convert {} into usize", self.type_hint()))),
//         }
//     }
// }

impl TryFrom<Value> for usize {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(b) => Ok(b as usize),
            Value::Int64(b) => Ok(b as usize),
            _ => Err(Error::new(format!("Cannot convert {} into usize", value.type_hint()))),
        }
    }
}

// impl TryInto<Option<usize>> for Value {

//     type Error = Error;

//     fn try_into(self) -> Result<Option<usize>, Self::Error> {
//         match self {
//             Value::Null => Ok(None),
//             Value::Int(b) => Ok(Some(b as usize)),
//             Value::Int64(b) => Ok(Some(b as usize)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<usize>", self.type_hint()))),
//         }
//     }
// }

impl TryFrom<Value> for Option<usize> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int(i) => Ok(Some(i as usize)),
            Value::Int64(i) => Ok(Some(i as usize)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<usize>", value.type_hint()))),
        }
    }
}

// impl TryInto<Option<usize>> for &Value {

//     type Error = Error;

//     fn try_into(self) -> Result<Option<usize>, Self::Error> {
//         match self {
//             Value::Null => Ok(None),
//             Value::Int(b) => Ok(Some(*b as usize)),
//             Value::Int64(b) => Ok(Some(*b as usize)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<usize>", self.type_hint()))),
//         }
//     }
// }

impl TryFrom<&Value> for Option<usize> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Int(i) => Ok(Some(*i as usize)),
            Value::Int64(i) => Ok(Some(*i as usize)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<usize>", value.type_hint()))),
        }
    }
}

