use chrono::NaiveDate;
use crate::error::Error;
use crate::value::Value;

impl TryInto<NaiveDate> for Value {

    type Error = Error;

    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        match self {
            Value::Date(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into NaiveDate", self.type_hint()))),
        }
    }
}

impl TryInto<NaiveDate> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        match self {
            Value::Date(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into NaiveDate", self.type_hint()))),
        }
    }
}

impl TryInto<Option<NaiveDate>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<NaiveDate>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Date(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<NaiveDate>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<NaiveDate>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<NaiveDate>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Date(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<NaiveDate>", self.type_hint()))),
        }
    }
}
