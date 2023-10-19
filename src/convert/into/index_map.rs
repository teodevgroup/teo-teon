use std::fmt::Display;
use indexmap::IndexMap;
use teo_result::Error;
use crate::value::Value;

impl<T> TryInto<IndexMap<String, T>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<IndexMap<String, T>, Self::Error> {
        match self {
            Value::Dictionary(map) => {
                let mut result: IndexMap<String, T> = IndexMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into IndexMap", self.type_hint()))),
        }
    }
}

impl <'a> TryInto<&'a IndexMap<String, Value>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<&'a IndexMap<String, Value>, Self::Error> {
        match self {
            Value::Dictionary(m) => Ok(m),
            _ => Err(Error::new(format!("Cannot convert {} into IndexMap", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<IndexMap<String, T>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<IndexMap<String, T>, Self::Error> {
        match self {
            Value::Dictionary(map) => {
                let mut result: IndexMap<String, T> = IndexMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into IndexMap", self.type_hint()))),
        }
    }
}

impl<T> TryInto<Option<IndexMap<String, T>>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<IndexMap<String, T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Dictionary(map) => {
                let mut result: IndexMap<String, T> = IndexMap::new();
                for (k, v) in map {
                    result.insert(k, v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<IndexMap>", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Option<IndexMap<String, T>>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<IndexMap<String, T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Dictionary(map) => {
                let mut result: IndexMap<String, T> = IndexMap::new();
                for (k, v) in map {
                    result.insert(k.to_owned(), v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<IndexMap>", self.type_hint()))),
        }
    }
}