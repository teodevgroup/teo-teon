use teo_result::Error;
use crate::Value;

impl<T0, T1> TryFrom<Value> for (T0, T1) where T0: TryFrom<Value, Error = Error>, T1: TryFrom<Value, Error = Error> {

    type Error = Error;

    fn try_from(ref value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tuple(values) => Ok((
                values.get(0).map(Clone::clone).ok_or(Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint())))?).unwrap().try_into()?,
                values.get(1).map(Clone::clone).ok_or(Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint())))?).unwrap().try_into()?,
            )),
            _ => Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint()))),
        }
    }
}

impl<T0, T1, T2> TryFrom<Value> for (T0, T1, T2) where T0: TryFrom<Value, Error = Error>, T1: TryFrom<Value, Error = Error>, T2: TryFrom<Value, Error = Error> {

    type Error = Error;

    fn try_from(ref value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tuple(values) => Ok((
                values.get(0).map(Clone::clone).ok_or(Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint())))?).unwrap().try_into()?,
                values.get(1).map(Clone::clone).ok_or(Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint())))?).unwrap().try_into()?,
                values.get(2).map(Clone::clone).ok_or(Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint())))?).unwrap().try_into()?,
            )),
            _ => Err(Error::new(format!("Cannot convert {} into Tuple", value.type_hint()))),
        }
    }
}