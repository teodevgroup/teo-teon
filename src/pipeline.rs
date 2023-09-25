use super::value::Value;

#[derive(Debug)]
pub struct TeonPipeline {
    items: Vec<TeonPipelineItem>
}

#[derive(Debug)]
pub struct TeonPipelineItem {
    path: Vec<String>,
    args: Vec<TeonPipelineItemArg>,
}

#[derive(Debug)]
pub struct TeonPipelineItemArg {
    name: Option<String>,
    arg: Value,
}