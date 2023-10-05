use std::collections::HashMap;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub items: Vec<PipelineItem>
}

#[derive(Debug, Clone)]
pub struct PipelineItem {
    pub path: Vec<usize>,
    pub args: HashMap<String, Value>,
}
