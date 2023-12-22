use teo_result::Error;
use crate::types::file::File;
use crate::value::Value;

impl TryFrom<Value> for File {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::File(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into File", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for File {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::File(s) => Ok(s.clone()),
            _ => Err(Error::new(format!("Cannot convert {} into File", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a File {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<&'a File, Self::Error> {
        match value {
            Value::File(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &File", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<File> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::File(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<File>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a File> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Option<&'a File>, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::File(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&File>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<File> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Option<File>, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::File(s) => Ok(Some(s.clone())),
            _ => Err(Error::new(format!("Cannot convert {} into Option<File>", value.type_hint()))),
        }
    }
}