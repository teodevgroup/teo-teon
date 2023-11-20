use teo_result::Error;
use crate::types::enum_variant::EnumVariant;
use crate::value::Value;

impl TryFrom<Value> for EnumVariant {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::EnumVariant(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into EnumVariant", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for EnumVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::EnumVariant(s) => Ok(s.clone()),
            _ => Err(Error::new(format!("Cannot convert {} into EnumVariant", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a EnumVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<&'a EnumVariant, Self::Error> {
        match value {
            Value::EnumVariant(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &EnumVariant", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<EnumVariant> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::EnumVariant(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<EnumVariant>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a EnumVariant> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Option<&'a EnumVariant>, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::EnumVariant(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&EnumVariant>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<EnumVariant> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Option<EnumVariant>, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::EnumVariant(s) => Ok(Some(s.clone())),
            _ => Err(Error::new(format!("Cannot convert {} into Option<EnumVariant>", value.type_hint()))),
        }
    }
}