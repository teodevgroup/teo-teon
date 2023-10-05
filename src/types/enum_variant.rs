use std::collections::HashMap;
use crate::value::Value;

pub struct EnumVariant {
    value: Value,
    display: String,
    path: Vec<usize>,
    args: Option<HashMap<String, Value>>,
}