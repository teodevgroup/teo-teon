use super::value::Value;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub items: Vec<PipelineItem>
}

#[derive(Debug, Clone)]
pub struct PipelineItem {
    pub path: Vec<String>,
    pub args: Vec<PipelineItemArg>,
}

#[derive(Debug, Clone)]
pub struct PipelineItemArg {
    pub name: Option<String>,
    pub arg: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub closed: bool,
    pub start: Box<Value>,
    pub end: Box<Value>,
}

#[derive(Debug, Clone)]
pub struct File {
    pub filepath: String,
    pub content_type: Option<String>,
    pub filename: String,
    pub filename_ext: Option<String>,
}

impl File {
    pub fn from_json_value(json_value: &serde_json::Value) -> Self {
        let obj = json_value.as_object().unwrap();
        Self {
            filepath: obj.get("filepath").unwrap().as_str().unwrap().to_owned(),
            content_type: obj.get("contentType").map(|c| if c.is_string() { Some(c.as_str().unwrap().to_owned()) } else { None }).flatten(),
            filename: obj.get("filename").unwrap().as_str().unwrap().to_owned(),
            filename_ext: obj.get("filenameExt").map(|c| if c.is_string() { Some(c.as_str().unwrap().to_owned()) } else { None }).flatten(),
        }
    }
}