use super::value::Value;

#[derive(Debug, Clone)]
pub struct TeonPipeline {
    items: Vec<TeonPipelineItem>
}

#[derive(Debug, Clone)]
pub struct TeonPipelineItem {
    path: Vec<String>,
    args: Vec<TeonPipelineItemArg>,
}

#[derive(Debug, Clone)]
pub struct TeonPipelineItemArg {
    name: Option<String>,
    arg: Value,
}