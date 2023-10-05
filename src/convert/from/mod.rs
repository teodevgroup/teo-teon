pub mod json;
pub mod str;
pub mod string;
pub mod bool;

use crate::value::Value;

impl From<&Value> for Value {

    fn from(v: &Value) -> Self {
        v.to_owned()
    }
}