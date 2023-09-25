use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TeonRange {
    pub(crate) closed: bool,
    pub(crate) start: Box<Value>,
    pub(crate) end: Box<Value>,
}