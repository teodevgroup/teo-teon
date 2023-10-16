use regex::Regex;
use crate::error::Error;
use crate::value::Value;


impl TryFrom<Value> for Regex {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Regex(r) => Ok(r),
            _ => Err(Error::new(format!("Cannot convert {} into Regex", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Regex {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Regex(r) => Ok(r.to_owned()),
            _ => Err(Error::new(format!("Cannot convert {} into Regex", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a Regex {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Regex(r) => Ok(r),
            _ => Err(Error::new(format!("Cannot convert {} into &Regex", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<Regex> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Regex(r) => Ok(Some(r)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Regex>", value.type_hint()))),
        }
    }
}


impl TryFrom<&Value> for Option<Regex> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Regex(_) => value.clone().try_into(),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Regex>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a Regex> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Regex(r) => Ok(Some(r)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&Regex>", value.type_hint()))),
        }
    }
}
