use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TeonRange {
    pub closed: bool,
    pub start: Box<Value>,
    pub end: Box<Value>,
}