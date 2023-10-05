use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub value: Box<Value>,
    pub display: String,
    pub path: Vec<usize>,
    pub args: Option<HashMap<String, Value>>,
}