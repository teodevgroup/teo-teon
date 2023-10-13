use crate::error::Error;
use crate::types::enum_variant::EnumVariant;
use crate::value::Value;

impl TryInto<EnumVariant> for Value {

    type Error = Error;

    fn try_into(self) -> Result<EnumVariant, Self::Error> {
        match self {
            Value::EnumVariant(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into EnumVariant", self.type_hint()))),
        }
    }
}

impl TryInto<EnumVariant> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<EnumVariant, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<&'a EnumVariant> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<&'a EnumVariant, Self::Error> {
        match self {
            Value::EnumVariant(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &EnumVariant", self.type_hint()))),
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

impl TryInto<Option<EnumVariant>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<EnumVariant>, Self::Error> {
        self.clone().try_into()
    }
}

impl<'a> TryInto<Option<&'a EnumVariant>> for &'a Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<&'a EnumVariant>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::EnumVariant(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&EnumVariant>", self.type_hint()))),
        }
    }
}