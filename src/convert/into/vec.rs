use std::fmt::Display;
use teo_result::Error;
use crate::value::Value;

impl<'a> TryInto<&'a Vec<Value>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<&'a Vec<Value>, Self::Error> {
        match self {
            Value::Array(vec) => Ok(vec),
            _ => Err(Error::new(format!("Cannot convert {} into &Array", self.type_hint()))),
        }
    }
}

impl<T> TryInto<Vec<T>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        match self {
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into Array", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Vec<T>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        match self {
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into Array", self.type_hint()))),
        }
    }
}

impl<T> TryInto<Option<Vec<T>>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<Vec<T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<Array>", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Option<Vec<T>>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<Vec<T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Array", self.type_hint()))),
        }
    }
}