use std::collections::HashMap;
use std::fmt::Display;
use crate::error::Error;
use crate::value::Value;

impl<T> TryInto<HashMap<String, T>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<HashMap<String, T>, Self::Error> {
        match self {
            Value::Dictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::BTreeDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::IndexDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into HashMap", self.type_hint()))),
        }
    }
}

impl <'a> TryInto<&'a HashMap<String, Value>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<&'a HashMap<String, Value>, Self::Error> {
        match self {
            Value::Dictionary(m) => Ok(m),
            _ => Err(Error::new(format!("Cannot convert {} into HashMap", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<HashMap<String, T>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<HashMap<String, T>, Self::Error> {
        match self {
            Value::Dictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::BTreeDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::IndexDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into HashMap", self.type_hint()))),
        }
    }
}

impl<T> TryInto<Option<HashMap<String, T>>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<HashMap<String, T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Dictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::BTreeDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::IndexDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<HashMap>", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Option<HashMap<String, T>>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<HashMap<String, T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Dictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::BTreeDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::IndexDictionary(map) => {
                let mut result: HashMap<String, T> = HashMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<HashMap>", self.type_hint()))),
        }
    }
}