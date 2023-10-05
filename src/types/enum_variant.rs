use std::collections::HashMap;
use std::ops::Not;
use bigdecimal::Zero;
use crate::error::Error;
use crate::value::Value;
use crate::result::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub value: Box<Value>,
    pub display: String,
    pub path: Vec<usize>,
    pub args: Option<HashMap<String, Value>>,
}

impl EnumVariant {

    pub fn normal_not(&self) -> Value {
        Value::Bool(match self.value.as_ref() {
            Value::Int(n) => n.is_zero(),
            Value::Int64(n) => n.is_zero(),
            Value::Float32(n) => n.is_zero(),
            Value::Float(n) => n.is_zero(),
            Value::Decimal(n) => n.is_zero(),
            _ => false,
        })
    }
}

impl Not for &EnumVariant {

    type Output = Result<Value>;

    fn not(self) -> Self::Output {
        if self.value.is_any_int() {
            Ok(self.value.as_ref().not()?)
        } else {
            Err(operand_error_message("bitneg"))
        }
    }
}

fn operand_error_message(name: &str) -> Error {
    Error::new(format!("cannot {name} EnumVariant"))
}

fn operands_error_message(name: &str) -> Error {
    Error::new(format!("cannot {name} with EnumVariant"))
}