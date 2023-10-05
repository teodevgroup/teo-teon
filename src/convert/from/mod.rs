pub mod json;
pub mod str;
pub mod string;
pub mod bool;
pub mod i32;
pub mod i64;
pub mod f32;
pub mod f64;
pub mod decimal;
pub mod object_id;
pub mod date;
pub mod datetime;

use crate::value::Value;

impl From<&Value> for Value {

    fn from(v: &Value) -> Self {
        v.to_owned()
    }
}