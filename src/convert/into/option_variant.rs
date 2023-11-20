use teo_result::Error;
use crate::types::option_variant::OptionVariant;
use crate::value::Value;

impl TryFrom<Value> for OptionVariant {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::OptionVariant(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into OptionVariant", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for OptionVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::OptionVariant(s) => Ok(s.clone()),
            _ => Err(Error::new(format!("Cannot convert {} into OptionVariant", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a OptionVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<&'a OptionVariant, Self::Error> {
        match value {
            Value::OptionVariant(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &OptionVariant", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<OptionVariant> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::OptionVariant(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<OptionVariant>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a OptionVariant> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Option<&'a OptionVariant>, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::OptionVariant(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&OptionVariant>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<OptionVariant> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Option<OptionVariant>, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::OptionVariant(s) => Ok(Some(s.clone())),
            _ => Err(Error::new(format!("Cannot convert {} into Option<OptionVariant>", value.type_hint()))),
        }
    }
}