use crate::error::Error;
use crate::types::enum_variant::EnumVariant;
use crate::value::Value;

impl TryFrom<Value> for EnumVariant {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::EnumVariant(e) => Ok(e),
            _ => Err(Error::new(format!("Cannot convert {} into EnumVarian", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for EnumVariant {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::EnumVariant(e) => Ok(e.to_owned()),
            _ => Err(Error::new(format!("Cannot convert {} into EnumVarian", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a EnumVariant {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::EnumVariant(r) => Ok(r),
            _ => Err(Error::new(format!("Cannot convert {} into &EnumVariant", value.type_hint()))),
        }
    }
}

impl TryInto<Option<EnumVariant>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<EnumVariant>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::EnumVariant(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<EnumVariant>", self.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<EnumVariant> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::EnumVariant(_) => value.clone().try_into(),
            _ => Err(Error::new(format!("Cannot convert {} into Option<EnumVariant>", value.type_hint()))),
        }
    }
}


impl<'a> TryFrom<&'a Value> for Option<&'a EnumVariant> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::EnumVariant(r) => Ok(Some(r)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&EnumVariant>", value.type_hint()))),
        }
    }
}
