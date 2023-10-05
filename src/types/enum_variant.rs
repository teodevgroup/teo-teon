use std::collections::HashMap;
use crate::value::Value;

pub struct EnumVariant {
    pub value: Value,
    pub display: String,
    pub path: Vec<usize>,
    pub args: Option<HashMap<String, Value>>,
}