use crate::error::Error;
use crate::types::range::Range;
use crate::value::Value;

impl TryFrom<Value> for Range {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Range(r) => Ok(r),
            _ => Err(Error::new(format!("Cannot convert {} into Range", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Range {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Range(r) => Ok(r.to_owned()),
            _ => Err(Error::new(format!("Cannot convert {} into Range", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a Range {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Range(r) => Ok(r),
            _ => Err(Error::new(format!("Cannot convert {} into &Range", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<Range> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Range(r) => Ok(Some(r)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Range>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<Range> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Range(_) => value.clone().try_into(),
            _ => Err(Error::new(format!("Cannot convert {} into Option<Range>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a Range> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Range(r) => Ok(Some(r)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&Range>", value.type_hint()))),
        }
    }
}