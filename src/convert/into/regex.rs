use regex::Regex;
use crate::error::Error;
use crate::value::Value;

impl TryInto<Regex> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Regex, Self::Error> {
        match self {
            Value::RegExp(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into Regex", self.type_hint()))),
        }
    }
}

impl TryInto<Regex> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Regex, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<&'a Regex> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<&'a Regex, Self::Error> {
        match self {
            Value::RegExp(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &Regex", self.type_hint()))),
        }
    }
}

impl TryInto<Option<Regex>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<Regex>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::RegExp(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Regex>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<Regex>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<Regex>, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<Option<&'a Regex>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<&'a Regex>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::RegExp(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&Regex>", self.type_hint()))),
        }
    }
}