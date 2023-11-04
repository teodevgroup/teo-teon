use crate::Value;
use teo_result::Error;

impl<'a> TryFrom<&'a Value> for Option<Value> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        Ok(Some(value.clone()))
    }
}