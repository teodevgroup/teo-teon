use bson::oid::ObjectId;
use crate::value::Value;

impl From<ObjectId> for Value {

    fn from(v: ObjectId) -> Self {
        Value::ObjectId(v)
    }
}

impl From<&ObjectId> for Value {

    fn from(v: &ObjectId) -> Self {
        Value::ObjectId(v.clone())
    }
}

impl From<Option<ObjectId>> for Value {

    fn from(v: Option<ObjectId>) -> Self {
        match v {
            Some(b) => Value::ObjectId(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&ObjectId>> for Value {

    fn from(v: Option<&ObjectId>) -> Self {
        match v {
            Some(b) => Value::ObjectId(b.clone()),
            None => Value::Null,
        }
    }
}