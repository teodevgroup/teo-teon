use super::value::Value;

#[derive(Debug, Clone)]
pub struct TeonPipeline {
    pub items: Vec<TeonPipelineItem>
}

#[derive(Debug, Clone)]
pub struct TeonPipelineItem {
    pub path: Vec<String>,
    pub args: Vec<TeonPipelineItemArg>,
}

#[derive(Debug, Clone)]
pub struct TeonPipelineItemArg {
    pub name: Option<String>,
    pub arg: Value,
}