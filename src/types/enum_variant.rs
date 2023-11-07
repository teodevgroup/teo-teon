use std::collections::{BTreeMap};
use serde::Serialize;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnumVariant {
    pub value: String,
    pub args: Option<BTreeMap<String, Value>>,
}

impl EnumVariant {

    pub fn into_string(self) -> String {
        self.value
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    pub fn normal_not(&self) -> bool {
        false
    }
}