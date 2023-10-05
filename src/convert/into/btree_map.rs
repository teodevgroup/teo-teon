use std::collections::BTreeMap;
use std::fmt::Display;
use crate::error::Error;
use crate::value::Value;

impl<T> TryInto<BTreeMap<String, T>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<BTreeMap<String, T>, Self::Error> {
        match self {
            Value::Dictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::BTreeDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::IndexDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into BTreeMap", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<BTreeMap<String, T>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<BTreeMap<String, T>, Self::Error> {
        match self {
            Value::Dictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::BTreeDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            Value::IndexDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into BTreeMap", self.type_hint()))),
        }
    }
}

impl<T> TryInto<Option<BTreeMap<String, T>>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<BTreeMap<String, T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Dictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::BTreeDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::IndexDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<HashMap>", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Option<BTreeMap<String, T>>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<BTreeMap<String, T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Dictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::BTreeDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            Value::IndexDictionary(map) => {
                let mut result: BTreeMap<String, T> = BTreeMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<BTreeMap>", self.type_hint()))),
        }
    }
}