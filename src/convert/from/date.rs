use chrono::NaiveDate;
use crate::value::Value;

impl From<NaiveDate> for Value {

    fn from(v: NaiveDate) -> Self {
        Value::Date(v)
    }
}

impl From<&NaiveDate> for Value {

    fn from(v: &NaiveDate) -> Self {
        Value::Date(*v)
    }
}

impl From<Option<NaiveDate>> for Value {

    fn from(v: Option<NaiveDate>) -> Self {
        match v {
            Some(b) => Value::Date(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&NaiveDate>> for Value {

    fn from(v: Option<&NaiveDate>) -> Self {
        match v {
            Some(b) => Value::Date(*b),
            None => Value::Null,
        }
    }
}