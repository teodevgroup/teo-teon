use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub closed: bool,
    pub start: Box<Value>,
    pub end: Box<Value>,
}