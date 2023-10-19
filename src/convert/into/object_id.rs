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
