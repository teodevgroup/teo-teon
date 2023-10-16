use crate::error::Error;
use crate::value::Value;

impl<'a> TryFrom<&'a Value> for &'a str {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s.as_str()),
            _ => Err(Error::new(format!("Cannot convert {} into &str", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a str> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::String(s) => Ok(Some(s.as_str())),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&str>", value.type_hint()))),
        }
    }
}
