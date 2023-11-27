use bson::oid::ObjectId;
use teo_result::Error;
use crate::value::Value;

impl TryInto<ObjectId> for Value {

    type Error = Error;

    fn try_into(self) -> Result<ObjectId, Self::Error> {
        match self {
            Value::ObjectId(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into ObjectId", self.type_hint()))),
        }
    }
}

impl TryInto<ObjectId> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<ObjectId, Self::Error> {
        match self {
            Value::ObjectId(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into ObjectId", self.type_hint()))),
        }
    }
}

impl TryInto<Option<ObjectId>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<ObjectId>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::ObjectId(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<ObjectId>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<ObjectId>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<ObjectId>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::ObjectId(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<ObjectId>", self.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a ObjectId {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::ObjectId(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into &ObjectId", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a ObjectId> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::ObjectId(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&ObjectId>", value.type_hint()))),
        }
    }
}
