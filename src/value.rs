use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::{Add, Div, Mul, Sub, Rem, Neg, BitAnd, BitXor, BitOr, Not};
use std::str::FromStr;
use chrono::prelude::{DateTime, Utc};
use indexmap::IndexMap;
use bson::oid::ObjectId;
use chrono::{NaiveDate, SecondsFormat};
use regex::Regex;
use bigdecimal::{BigDecimal, Zero};
use itertools::Itertools;
use crate::types::enum_variant::EnumVariant;
use crate::types::file::File;
use crate::types::pipeline::Pipeline;
use crate::types::range::Range;
use super::index::Index;
use super::error::Error;
use super::result::Result;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/mod.rs

/// Represents any valid Teon value.
///
#[derive(Debug, Clone)]
pub enum Value {

    /// Represents an undetermined value. This is typically returned from parser when error occurs
    ///
    Undetermined,

    /// Represents a Teon null value.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(null);
    /// ```
    Null,

    /// Represents a Teon Bool.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(true);
    /// ```
    Bool(bool),

    /// Represents a Teon Int.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12_i32);
    /// ```
    Int(i32),

    /// Represents a Teon Int64.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12_i64);
    /// ```
    Int64(i64),

    /// Represents a Teon Float32.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12.5_f32);
    /// ```
    Float32(f32),

    /// Represents a Teon Float.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12.5_f64);
    /// ```
    Float(f64),

    /// Represents a Teon Decimal.
    ///
    Decimal(BigDecimal),

    /// Represents a Teon ObjectId.
    ///
    ObjectId(ObjectId),

    /// Represents a Teon String.
    ///
    String(String),

    /// Represents a Teon Date.
    ///
    Date(NaiveDate),

    /// Represents a Teon DateTime.
    ///
    DateTime(DateTime<Utc>),

    /// Represents a Teon Array.
    ///
    Array(Vec<Value>),

    /// Represents a Teon Dictionary.
    ///
    Dictionary(HashMap<String, Value>),

    /// Represents a Teon btree_dictionary.
    ///
    BTreeDictionary(BTreeMap<String, Value>),

    /// Represents a Teon index_dictionary.
    ///
    IndexDictionary(IndexMap<String, Value>),

    /// Represents a Teon Range.
    ///
    Range(Range),

    /// Represents a Teon Tuple.
    ///
    Tuple(Vec<Value>),

    /// Represents a Teon enum variant value.
    ///
    EnumVariant(EnumVariant),

    /// Represents a Teon RegExp.
    ///
    RegExp(Regex),

    /// Represents a Teon File.
    ///
    File(File),

    /// Represents a Teon Pipeline.
    ///
    Pipeline(Pipeline),

    /// Represents a Teon Reference. Its type is determined by the parser.
    ///
    Reference(Vec<usize>),
}

impl Value {

    // Access
    
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    // Value

    pub fn is_undetermined(&self) -> bool {
        match self {
            Value::Undetermined => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        self.as_int().is_some()
    }

    pub fn as_int(&self) -> Option<i32> {
        match *self {
            Value::Int(v) => Some(v),
            _ => None
        }
    }

    pub fn to_int(&self) -> Option<i32> {
        match *self {
            Value::Int(i) => Some(i),
            Value::Int64(i) => if i >= (i32::MAX as i64) {
                None
            } else {
                Some(i as i32)
            }
            _ => None
        }
    }

    pub fn is_int64(&self) -> bool {
        self.as_int64().is_some()
    }

    pub fn as_int64(&self) -> Option<i64> {
        match *self {
            Value::Int64(v) => Some(v),
            _ => None
        }
    }

    pub fn to_int64(&self) -> Option<i64> {
        match *self {
            Value::Int64(v) => Some(v),
            Value::Int(v) => Some(v as i64),
            _ => None,
        }
    }

    pub fn is_float32(&self) -> bool {
        self.as_float32().is_some()
    }

    pub fn as_float32(&self) -> Option<f32> {
        match *self {
            Value::Float32(v) => Some(v),
            _ => None
        }
    }

    pub fn to_float32(&self) -> Option<f32> {
        match *self {
            Value::Float32(v) => Some(v),
            Value::Float(v) => Some(v as f32),
            Value::Int(i) => Some(i as f32),
            Value::Int64(i) => Some(i as f32),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(v) => Some(v),
            _ => None
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        match *self {
            Value::Int(v) => Some(v as f64),
            Value::Int64(v) => Some(v as f64),
            Value::Float32(v) => Some(v as f64),
            Value::Float(v) => Some(v),
            _ => None
        }
    }

    pub fn is_decimal(&self) -> bool {
        match *self {
            Value::Decimal(_) => true,
            _ => false,
        }
    }

    pub fn as_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Value::Decimal(v) => Some(v),
            _ => None
        }
    }

    pub fn is_object_id(&self) -> bool {
        self.as_object_id().is_some()
    }

    pub fn as_object_id(&self) -> Option<&ObjectId> {
        match self {
            Value::ObjectId(o) => Some(o),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_date(&self) -> bool {
        self.as_date().is_some()
    }

    pub fn as_date(&self) -> Option<&NaiveDate> {
        match self {
            Value::Date(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }

    pub fn as_datetime(&self) -> Option<&DateTime<Utc>> {
        match self {
            Value::DateTime(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn is_dictionary(&self) -> bool {
        self.as_dictionary().is_some()
    }

    pub fn as_dictionary(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_dictionary_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::Dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_btree_dictionary(&self) -> bool {
        self.as_btree_dictionary().is_some()
    }

    pub fn as_btree_dictionary(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::BTreeDictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_btree_dictionary_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match self {
            Value::BTreeDictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_index_dictionary(&self) -> bool {
        self.as_index_dictionary().is_some()
    }

    pub fn as_index_dictionary(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::IndexDictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_index_dictionary_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::IndexDictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_range(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_range(&self) -> Option<&Range> {
        match self {
            Value::Range(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_tuple(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_tuple(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Tuple(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_enum_variant(&self) -> bool {
        self.as_enum_variant().is_some()
    }

    pub fn as_enum_variant(&self) -> Option<&EnumVariant> {
        match self {
            Value::EnumVariant(e) => Some(e),
            _ => None,
        }
    }

    pub fn is_regexp(&self) -> bool {
        self.as_regexp().is_some()
    }

    pub fn as_regexp(&self) -> Option<&Regex> {
        match self {
            Value::RegExp(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_file(&self) -> bool {
        self.as_file().is_some()
    }

    pub fn as_file(&self) -> Option<&File> {
        match self {
            Value::File(f) => Some(f),
            _ => None,
        }
    }

    pub fn is_pipeline(&self) -> bool {
        self.as_pipeline().is_some()
    }

    pub fn as_pipeline(&self) -> Option<&Pipeline> {
        match self {
            Value::Pipeline(p) => Some(p),
            _ => None,
        }
    }

    pub fn is_reference(&self) -> bool {
        self.as_reference().is_some()
    }

    pub fn as_reference(&self) -> Option<&Vec<usize>> {
        match self {
            Value::Reference(r) => Some(r),
            _ => None,
        }
    }

    // Compound queries

    pub fn is_any_int(&self) -> bool {
        match *self {
            Value::Int(_) | Value::Int64(_) => true,
            _ => false,
        }
    }

    pub fn is_any_float(&self) -> bool {
        match *self {
            Value::Float32(_) | Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_any_int_or_float(&self) -> bool {
        self.is_any_int() || self.is_any_float()
    }

    pub fn is_any_number(&self) -> bool {
        self.is_any_int() || self.is_any_float() || self.is_decimal()
    }

    pub fn to_usize(&self) -> Option<usize> {
        match *self {
            Value::Int(n) => Some(n as usize),
            Value::Int64(n) => Some(n as usize),
            _ => None
        }
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    // Type hint

    pub fn type_hint(&self) -> &str {
        match self {
            Value::Undetermined => "Undetermined",
            Value::Null => "Null",
            Value::Bool(_) => "Bool",
            Value::Int(_) => "Int",
            Value::Int64(_) => "Int64",
            Value::Float32(_) => "Float32",
            Value::Float(_) => "Float",
            Value::Decimal(_) => "Decimal",
            Value::ObjectId(_) => "ObjectId",
            Value::String(_) => "String",
            Value::Date(_) => "Date",
            Value::DateTime(_) => "DateTime",
            Value::Array(_) => "Array",
            Value::Dictionary(_) => "Dictionary",
            Value::BTreeDictionary(_) => "BTreeDictionary",
            Value::IndexDictionary(_) => "IndexDictionary",
            Value::Range(_) => "Range",
            Value::Tuple(_) => "Tuple",
            Value::EnumVariant(e) => {
                if e.value.is_any_int() {
                    "OptionEnumVariant"
                } else {
                    "EnumVariant"
                }
            },
            Value::RegExp(_) => "RegExp",
            Value::File(_) => "File",
            Value::Pipeline(_) => "Pipeline",
            Value::Reference(_) => "Reference",
        }
    }

    pub fn recip(&self) -> Result<Value> {
        Ok(match self {
            Value::Int(n) => Value::Float((*n as f64).recip()),
            Value::Int64(n) => Value::Float((*n as f64).recip()),
            Value::Float32(n) => Value::Float32((*n).recip()),
            Value::Float(n) => Value::Float((*n).recip()),
            Value::Decimal(n) => Value::Decimal(BigDecimal::from_str("1").unwrap() / n),
            _ => Err(Error::new("recip: value is not number"))?
        })
    }

    pub fn normal_not(&self) -> Value {
        Value::Bool(match self {
            Value::Undetermined => true,
            Value::Null => true,
            Value::Bool(b) => !b,
            Value::Int(i) => i.is_zero(),
            Value::Int64(i) => i.is_zero(),
            Value::Float32(f) => f.is_zero(),
            Value::Float(f) => f.is_zero(),
            Value::Decimal(d) => d.is_zero(),
            Value::ObjectId(o) => false,
            Value::String(s) => s.is_empty(),
            Value::Date(_) => false,
            Value::DateTime(_) => false,
            Value::Array(a) => a.is_empty(),
            Value::Dictionary(d) => d.is_empty(),
            Value::BTreeDictionary(d) => d.is_empty(),
            Value::IndexDictionary(d) => d.is_empty(),
            Value::Range(_) => false,
            Value::Tuple(a) => false,
            Value::EnumVariant(e) => (e.normal_not()).as_bool().unwrap(),
            Value::RegExp(_) => false,
            Value::File(_) => false,
            Value::Pipeline(_) => false,
            Value::Reference(_) => false,
        })
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

fn check_operand<F>(operand: &Value, name: &str, matcher: F) -> Result<()> where F: Fn(&Value) -> bool {
    if !matcher(operand) {
        return Err(operand_error_message(operand, name));
    }
    Ok(())
}

fn operand_error_message(operand: &Value, name: &str) -> Error {
    Error::new(format!("cannot {name} {}", operand.type_hint()))
}

fn check_operands<F>(lhs: &Value, rhs: &Value, name: &str, matcher: F) -> Result<()> where F: Fn(&Value) -> bool {
    let matcher_wrapper = |value: &Value| {
        (&matcher)(value)
    };
    if !matcher_wrapper(lhs) || !matcher_wrapper(rhs) {
        return Err(operands_error_message(lhs, rhs, name));
    }
    Ok(())
}

fn operands_error_message(lhs: &Value, rhs: &Value, name: &str) -> Error {
    Error::new(format!("cannot {name} {} with {}", lhs.type_hint(), rhs.type_hint()))
}

impl Add for &Value {

    type Output = Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int())?;
                Value::Int(v + rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int())?;
                Value::Int64(v + rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int_or_float())?;
                Value::Float32(v + rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "add", |v| v.is_any_int_or_float())?;
                Value::Float(v + rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "add", |v| v.is_decimal())?;
                Value::Decimal(d + rhs.as_decimal().unwrap())
            },
            Value::String(s) => {
                check_operands(&self, &rhs, "add", |v| v.is_string())?;
                Value::String(s.to_owned() + rhs.as_str().unwrap())
            }
            _ => Err(operands_error_message(self, rhs, "add"))?,
        })
    }
}

impl Sub for &Value {

    type Output = Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int())?;
                Value::Int(v - rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int())?;
                Value::Int64(v - rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int_or_float())?;
                Value::Float32(v - rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "sub", |v| v.is_any_int_or_float())?;
                Value::Float(v - rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "sub", |v| v.is_decimal())?;
                Value::Decimal(d - rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "sub"))?,
        })
    }
}

impl Mul for &Value {

    type Output = Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int())?;
                Value::Int(v * rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int())?;
                Value::Int64(v * rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int_or_float())?;
                Value::Float32(v * rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "mul", |v| v.is_any_int_or_float())?;
                Value::Float(v * rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "mul", |v| v.is_decimal())?;
                Value::Decimal(d * rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "mul"))?,
        })
    }
}

impl Div for &Value {

    type Output = Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int())?;
                Value::Int(v / rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int())?;
                Value::Int64(v / rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int_or_float())?;
                Value::Float32(v / rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "div", |v| v.is_any_int_or_float())?;
                Value::Float(v / rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "div", |v| v.is_decimal())?;
                Value::Decimal(d / rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "div"))?,
        })
    }
}

impl Rem for &Value {

    type Output = Result<Value>;

    fn rem(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int())?;
                Value::Int(v % rhs.to_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int())?;
                Value::Int64(v % rhs.to_int64().unwrap())
            },
            Value::Float32(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int_or_float())?;
                Value::Float32(v % rhs.to_float32().unwrap())
            },
            Value::Float(v) => {
                check_operands(&self, &rhs, "rem", |v| v.is_any_int_or_float())?;
                Value::Float(v % rhs.to_float().unwrap())
            },
            Value::Decimal(d) => {
                check_operands(&self, &rhs, "rem", |v| v.is_decimal())?;
                Value::Decimal(d % rhs.as_decimal().unwrap())
            },
            _ => Err(operands_error_message(self, rhs, "rem"))?,
        })
    }
}

impl Neg for &Value {

    type Output = Result<Value>;

    fn neg(self) -> Self::Output {
        Ok(match self {
            Value::Int(val) => Value::Int(-*val),
            Value::Int64(val) => Value::Int64(-*val),
            Value::Float32(val) => Value::Float32(-*val),
            Value::Float(val) => Value::Float(-*val),
            Value::Decimal(val) => Value::Decimal(val.neg()),
            _ => Err(operand_error_message(self, "neg"))?,
        })
    }
}

impl BitAnd for &Value {

    type Output = Result<Value>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "bitand", |v| v.is_any_int())?;
                Value::Int(v & rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "bitand", |v| v.is_any_int())?;
                Value::Int64(v & rhs.as_int64().unwrap())
            },
            _ => Err(operand_error_message(self, "bitand"))?,
        })
    }
}

impl BitXor for &Value {

    type Output = Result<Value>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "bitxor", |v| v.is_any_int())?;
                Value::Int(v ^ rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "bitxor", |v| v.is_any_int())?;
                Value::Int64(v ^ rhs.as_int64().unwrap())
            },
            _ => Err(operand_error_message(self, "bitxor"))?,
        })
    }
}

impl BitOr for &Value {

    type Output = Result<Value>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Ok(match self {
            Value::Int(v) => {
                check_operands(&self, rhs, "bitor", |v| v.is_any_int())?;
                Value::Int(v | rhs.as_int().unwrap())
            },
            Value::Int64(v) => {
                check_operands(&self, rhs, "bitor", |v| v.is_any_int())?;
                Value::Int64(v | rhs.as_int64().unwrap())
            },
            _ => Err(operand_error_message(self, "bitor"))?,
        })
    }
}

// This is bit neg
impl Not for &Value {

    type Output = Result<Value>;

    fn not(self) -> Self::Output {
        Ok(match self {
            Value::Int(val) => Value::Int(-*val),
            Value::Int64(val) => Value::Int64(-*val),
            Value::Float32(val) => Value::Float32(-*val),
            Value::Float(val) => Value::Float(-*val),
            Value::Decimal(val) => Value::Decimal(val.neg()),
            Value::EnumVariant(e) => e.not()?,
            _ => Err(operand_error_message(self, "bitneg"))?,
        })
    }
}

impl PartialEq for Value {

    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        if self.is_any_int() && other.is_any_int() {
            return self.to_int64().unwrap() == other.to_int64().unwrap();
        }
        if self.is_any_int_or_float() && other.is_any_int_or_float() {
            return self.to_float().unwrap() == other.to_float().unwrap();
        }
        match (self, other) {
            (Null, Null) => true,
            (Bool(s), Bool(o)) => s == o,
            (Decimal(s), Decimal(o)) => s == o,
            (ObjectId(s), ObjectId(o)) => s == o,
            (String(s), String(o)) => s == o,
            (Date(s), Date(o)) => s == o,
            (DateTime(s), DateTime(o)) => s == o,
            (Array(s), Array(o)) => s == o,
            (Dictionary(s), Dictionary(o)) => s == o,
            (BTreeDictionary(s), BTreeDictionary(o)) => s == o,
            (IndexDictionary(s), IndexDictionary(o)) => s == o,
            (Range(s), Range(o)) => s == o,
            (Tuple(s), Tuple(o)) => s == o,
            (EnumVariant(s), EnumVariant(o)) => s == o,
            (RegExp(s), RegExp(o)) => s.as_str() == o.as_str(),
            (File(s), File(o)) => s == o,
            (Pipeline(s), Pipeline(o)) => s == o,
            (Reference(s), Reference(o)) => s == o,
            _ => false,
        }
    }
}

impl PartialOrd for Value {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;
        if self.is_any_int() && other.is_any_int() {
            return self.to_int64().unwrap().partial_cmp(&other.to_int64().unwrap());
        }
        if self.is_any_int_or_float() && other.is_any_int_or_float() {
            return self.to_float().unwrap().partial_cmp(&other.to_float().unwrap());
        }
        match (self, other) {
            (Null, Null) => Some(Ordering::Equal),
            (Bool(s), Bool(o)) => s.partial_cmp(o),
            (Decimal(s), Decimal(o)) => s.partial_cmp(o),
            (ObjectId(s), ObjectId(o)) => s.partial_cmp(o),
            (String(s), String(o)) => s.partial_cmp(o),
            (Date(s), Date(o)) => s.partial_cmp(o),
            (DateTime(s), DateTime(o)) => s.partial_cmp(o),
            (Array(s), Array(o)) => s.partial_cmp(o),
            (Tuple(s), Tuple(o)) => s.partial_cmp(o),
            (EnumVariant(s), EnumVariant(o)) => s.value.partial_cmp(&o.value),
            _ => None,
        }
    }
}

impl AsRef<Value> for Value {

    fn as_ref(&self) -> &Value {
        &self
    }
}

impl Display for Value {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Undetermined => f.write_str("undetermined"),
            Value::Null => f.write_str("null"),
            Value::Bool(b) => Display::fmt(b, f),
            Value::Int(i) => Display::fmt(i, f),
            Value::Int64(i) => Display::fmt(i, f),
            Value::Float32(n) => Display::fmt(n, f),
            Value::Float(n) => Display::fmt(n, f),
            Value::Decimal(d) => {
                f.write_str("Decimal(\"")?;
                Display::fmt(d, f)?;
                f.write_str("\"")
            },
            Value::ObjectId(o) => {
                f.write_str("ObjectId(\"")?;
                Display::fmt(o, f)?;
                f.write_str("\"")
            },
            Value::String(s) => {
                f.write_str(&format!("\"{}\"", s.replace("\"", "\\\"")))
            }
            Value::Date(d) => f.write_str(&format!("Date(\"{}\")", d.to_string())),
            Value::DateTime(d) => f.write_str(&format!("DateTime(\"{}\")", d.to_rfc3339_opts(SecondsFormat::Millis, true))),
            Value::Array(a) => {
                f.write_str(&("[".to_string() + a.iter().map(|v| format!("{v}")).join(", ").as_str() + "]"))
            }
            Value::Dictionary(m) => {
                f.write_str(&("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", format!("{v}"))).join(", ").as_str() + "}"))
            }
            Value::BTreeDictionary(m) => {
                f.write_str(&("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", format!("{v}"))).join(", ").as_str() + "}"))
            }
            Value::IndexDictionary(m) => {
                f.write_str(&("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", format!("{v}"))).join(", ").as_str() + "}"))
            }
            Value::Range(r) => Display::fmt(r, f),
            Value::Tuple(t) => {
                f.write_str("(")?;
                for (i, v) in t.iter().enumerate() {
                    Display::fmt(v, f)?;
                    if i != t.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
                if t.len() == 1 {
                    f.write_str(",")?;
                }
                f.write_str(")")
            }
            Value::EnumVariant(e) => {
                f.write_str(&e.display)
            }
            Value::RegExp(r) => {
                f.write_str("/")?;
                f.write_str(&format!("{}", r.as_str().replace("/", "\\/")))?;
                f.write_str("/")
            }
            Value::File(file) => Display::fmt(file, f),
            Value::Pipeline(p) => Display::fmt(p, f),
            Value::Reference(r) => {
                f.write_str("Reference(\"")?;
                f.write_str(&r.iter().map(|s| s.to_string()).join(", "))?;
                f.write_str("\")")
            }
        }
    }
}
