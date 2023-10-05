pub mod json;

use crate::value::Value;

impl From<&Value> for Value {

    fn from(v: &Value) -> Self {
        v.to_owned()
    }
}