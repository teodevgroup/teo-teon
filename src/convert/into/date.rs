use chrono::NaiveDate;
use teo_result::Error;
use crate::value::Value;

impl TryFrom<Value> for NaiveDate {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Date(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into NaiveDate", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for NaiveDate {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Date(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into NaiveDate", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a NaiveDate {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Date(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into &NaiveDate", value.type_hint()))),
        }
    }
}


impl TryFrom<Value> for Option<NaiveDate> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Date(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<NaiveDate>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<NaiveDate> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Date(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<NaiveDate>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a NaiveDate> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Date(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&NaiveDate>", value.type_hint()))),
        }
    }
}

