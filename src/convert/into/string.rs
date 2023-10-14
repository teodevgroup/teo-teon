use crate::error::Error;
use crate::value::Value;

impl TryInto<String> for Value {

    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into String", self.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for String {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s.to_owned()),
            _ => Err(Error::new(format!("Cannot convert {} into String", value.type_hint()))),
        }
    }
}

impl TryInto<Option<String>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<String>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::String(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<String>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<String>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<String>, Self::Error> {
        self.clone().try_into()
    }
}