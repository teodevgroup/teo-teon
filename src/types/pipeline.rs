use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub items: Vec<PipelineItem>
}

#[derive(Debug, Clone)]
pub struct PipelineItem {
    pub path: Vec<usize>,
    pub string_path: Vec<String>,
    pub args: Option<HashMap<String, Value>>,
}

impl Display for Pipeline {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, item) in self.items.iter().enumerate() {
            if index == 0 {
                f.write_str("$")?;
            } else {
                f.write_str(".")?;
            }
            f.write_str(&item.string_path.join("."))?;
            if let Some(args) = &item.args {
                f.write_str("(")?;
                f.write_str(&args.iter().map(|k, v| format!("{k}: {}", v)).join(", "))?;
                f.write_str(")")?;
            }
        }
        Ok(())
    }
}