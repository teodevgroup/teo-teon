use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use bigdecimal::Zero;
use serde::Serialize;
use teo_result::Error;
use crate::value::Value;
use teo_result::Result;

#[derive(Debug, Clone, PartialEq, Serialize)]
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

    pub fn is_option(&self) -> bool {
        self.value.is_any_int()
    }

    fn check_operand(&self, other: &Self, name: &str) -> Result<()> {
        if self.is_option() && other.path == self.path {
            Ok(())
        } else {
            Err(operands_error_message(name))
        }
    }
}

impl BitAnd for &EnumVariant {

    type Output = Result<EnumVariant>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.check_operand(rhs, "bitor")?;
        Ok(EnumVariant {
            value: Box::new((self.value.as_ref() & rhs.value.as_ref())?),
            display: format!("({} & {})", self.display, rhs.display),
            path: self.path.clone(),
            args: None,
        })
    }
}

impl BitOr for &EnumVariant {

    type Output = Result<EnumVariant>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.check_operand(rhs, "bitor")?;
        Ok(EnumVariant {
            value: Box::new((self.value.as_ref() | rhs.value.as_ref())?),
            display: format!("({} | {})", self.display, rhs.display),
            path: self.path.clone(),
            args: None,
        })
    }
}

impl BitXor for &EnumVariant {

    type Output = Result<EnumVariant>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.check_operand(rhs, "bitxor")?;
        Ok(EnumVariant {
            value: Box::new((self.value.as_ref() ^ rhs.value.as_ref())?),
            display: format!("({} ^ {})", self.display, rhs.display),
            path: self.path.clone(),
            args: None,
        })
    }
}

impl Not for &EnumVariant {

    type Output = Result<EnumVariant>;

    fn not(self) -> Self::Output {
        if self.value.is_any_int() {
            Ok(EnumVariant {
                value: Box::new(self.value.as_ref().not()?),
                display: format!("~{}", self.display),
                path: self.path.clone(),
                args: None,
            })
        } else {
            Err(operand_error_message("bitneg"))
        }
    }
}

fn operand_error_message(name: &str) -> Error {
    Error::new(format!("cannot {name}"))
}

fn operands_error_message(name: &str) -> Error {
    Error::new(format!("cannot {name}"))
}