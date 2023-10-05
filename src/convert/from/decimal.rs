use bigdecimal::BigDecimal;
use crate::value::Value;

impl From<BigDecimal> for Value {

    fn from(v: BigDecimal) -> Self {
        Value::Decimal(v)
    }
}

impl From<&BigDecimal> for Value {

    fn from(v: &BigDecimal) -> Self {
        Value::Decimal(v.clone())
    }
}

impl From<Option<BigDecimal>> for Value {

    fn from(v: Option<BigDecimal>) -> Self {
        match v {
            Some(b) => Value::Decimal(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&BigDecimal>> for Value {

    fn from(v: Option<&BigDecimal>) -> Self {
        match v {
            Some(b) => Value::Decimal(b.clone()),
            None => Value::Null,
        }
    }
}