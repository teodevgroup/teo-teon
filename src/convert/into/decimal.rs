use bigdecimal::BigDecimal;
use crate::error::Error;
use crate::value::Value;

impl TryFrom<&Value> for BigDecimal {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal(b) => Ok(b.clone()),
            _ => Err(Error::new(format!("Cannot convert {} into BigDecimal", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for BigDecimal {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into BigDecimal", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a BigDecimal {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &BigDecimal", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<BigDecimal> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Decimal(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<BigDecimal>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<BigDecimal> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Decimal(b) => Ok(Some(b.clone())),
            _ => Err(Error::new(format!("Cannot convert {} into Option<BigDecimal>", value.type_hint()))),
        }
    }
}

//-------------------- test --------------------
// impl<'a> TryFrom<Value> for &'a BigDecimal {

//     type Error = Error;

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         match value {
//             Value::BigDecimal(d) => Ok(d),
//             _ => Err(Error::new(format!("Cannot convert {} into &BigDecimal", value.type_hint()))),
//         }
//     }
// }

// impl TryFrom<Value> for Option<&BigDecimal> {

//     type Error = Error;

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         match value {
//             Value::Null => Ok(None),
//             Value::Decimal(d) => Ok(Some(&d)),
//             _ => Err(Error::new(format!("Cannot convert {} into Option<&BigDecimal>", value.type_hint()))),
//         }
//     }
// }
//-------------------- test --------------------

impl<'a> TryFrom<&'a Value> for Option<&'a BigDecimal> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::Decimal(d) => Ok(Some(d)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&BigDecimal>", value.type_hint()))),
        }
    }
}

