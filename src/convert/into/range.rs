use crate::error::Error;
use crate::types::range::Range;
use crate::value::Value;

impl TryInto<Range> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Range, Self::Error> {
        match self {
            Value::Range(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into Range", self.type_hint()))),
        }
    }
}

impl TryInto<Range> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Range, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<&'a Range> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<&'a Range, Self::Error> {
        match self {
            Value::Range(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &Range", self.type_hint()))),
        }
    }
}

impl TryInto<Option<Range>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<Range>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Range(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Range>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<Range>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<Range>, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<Option<&'a Range>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<&'a Range>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Range(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&Range>", self.type_hint()))),
        }
    }
}