use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::{Add, Div, Mul, Sub, Rem, Neg, BitAnd, BitXor, BitOr};
use chrono::prelude::{DateTime, Utc};
use indexmap::IndexMap;
use bson::oid::ObjectId;
use chrono::{NaiveDate, SecondsFormat};
use regex::Regex;
use bigdecimal::BigDecimal;
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
            Value::btree_dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_btree_dictionary_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match self {
            Value::btree_dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_index_dictionary(&self) -> bool {
        self.as_index_dictionary().is_some()
    }

    pub fn as_index_dictionary(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::index_dictionary(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_index_dictionary_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::index_dictionary(map) => Some(map),
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

    pub fn recip(&self) -> f64 {
        match self {
            Value::Int(n) => (*n as f64).recip(),
            Value::Int64(n) => (*n as f64).recip(),
            Value::Float32(n) => (*n as f64).recip(),
            Value::Float(n) => (*n as f64).recip(),
            Value::Decimal(_n) => panic!("decimal div todo"),
            _ => panic!()
        }
    }

    pub fn fmt_for_display(&self) -> Cow<str> {
        match self {
            Value::Null => Cow::Borrowed("null"),
            Value::Bool(v) => if *v { Cow::Borrowed("true") } else { Cow::Borrowed("false") },
            Value::Int(i) => Cow::Owned(i.to_string()),
            Value::Int64(i) => Cow::Owned(i.to_string()),
            Value::Float32(f) => Cow::Owned(f.to_string()),
            Value::Float(f) => Cow::Owned(f.to_string()),
            Value::Decimal(d) => Cow::Owned(format!("Decimal(\"{}\")", d.to_string())),
            Value::ObjectId(o) => Cow::Owned(format!("ObjectId(\"{}\")", o.to_hex())),
            Value::String(s) => Cow::Owned(format!("\"{}\"", s.replace("\"", "\\\""))),
            Value::Date(d) => Cow::Owned(format!("Date(\"{}\")", d.to_string())),
            Value::DateTime(dt) => Cow::Owned(format!("DateTime(\"{}\")", dt.to_rfc3339_opts(SecondsFormat::Millis, true))),
            Value::Array(v) => Cow::Owned("[".to_string() + v.iter().map(|v| v.fmt_for_display()).join(", ").as_str() + "]"),
            Value::Dictionary(m) => Cow::Owned("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", v.fmt_for_display())).join(", ").as_str() + "}"),
            Value::btree_dictionary(m) => Cow::Owned("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", v.fmt_for_display())).join(", ").as_str() + "}"),
            Value::index_dictionary(m) => Cow::Owned("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", v.fmt_for_display())).join(", ").as_str() + "}"),
            Value::Range(_) => unreachable!(),
            Value::Tuple(v) => Cow::Owned("(".to_string() + v.iter().map(|v| v.fmt_for_display()).join(", ").as_str() + ")"),
            Value::Pipeline(_) => unreachable!(),
            Value::EnumVariant(v, _) => Cow::Owned(format!(".{}", v.as_str())),
            Value::OptionVariant(_) => unreachable!(),
            Value::RegExp(_) => unreachable!(),
            Value::Reference(_) => unreachable!(),
            Value::File(f) => Cow::Owned(format!("File(\"{}\")", f.filename)),
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;
        match (self, other) {
            (Null, Null) => Some(Ordering::Equal),
            (ObjectId(s), ObjectId(o)) => s.partial_cmp(o),
            (Bool(s), Bool(o)) => s.partial_cmp(o),
            (Int(s), Int(o)) => s.partial_cmp(o),
            (Int64(s), Int64(o)) => s.partial_cmp(o),
            (Float32(s), Float32(o)) => s.partial_cmp(o),
            (Float(s), Float(o)) => s.partial_cmp(o),
            (Decimal(s), Decimal(o)) => s.partial_cmp(o),
            (String(s), String(o)) => s.partial_cmp(o),
            (Date(s), Date(o)) => s.partial_cmp(o),
            (DateTime(s), DateTime(o)) => s.partial_cmp(o),
            (Array(s), Array(o)) => s.partial_cmp(o),
            (Dictionary(_s), Dictionary(_o)) => None,
            (btree_dictionary(_s), btree_dictionary(_o)) => None,
            _ => None,
        }
    }
}

fn check_operand(lhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_any_number() {
        return Err(Error::new(format!("{}: operand is not number", name)));
    }
    Ok(())
}

fn check_operands(lhs: &Value, rhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_any_number() {
        return Err(Error::new(format!("{}: lhs is not number", name)));
    }
    if !rhs.is_any_number() {
        return Err(Error::new(format!("{}: rhs is not number", name)));
    }
    Ok(())
}

fn check_operands_int(lhs: &Value, rhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_any_int() {
        return Err(Error::new(format!("{}: lhs is not number", name)));
    }
    if !rhs.is_any_int() {
        return Err(Error::new(format!("{}: rhs is not number", name)));
    }
    Ok(())
}

impl Add for Value {
    type Output = Result<Value>;
    fn add(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "add")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v + rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v + rhs.as_int64().unwrap()),
            Value::Float32(v) => Value::Float32(v + rhs.as_float32().unwrap()),
            Value::Float(v) => Value::Float(v + rhs.as_float().unwrap()),
            Value::Decimal(d) => Value::Decimal(d + rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Sub for Value {
    type Output = Result<Value>;
    fn sub(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "sub")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v - rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v - rhs.as_int64().unwrap()),
            Value::Float32(v) => Value::Float32(v - rhs.as_float32().unwrap()),
            Value::Float(v) => Value::Float(v - rhs.as_float().unwrap()),
            Value::Decimal(d) => Value::Decimal(d - rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Mul for Value {
    type Output = Result<Value>;
    fn mul(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "mul")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v * rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v * rhs.as_int64().unwrap()),
            Value::Float32(v) => Value::Float32(v * rhs.as_float32().unwrap()),
            Value::Float(v) => Value::Float(v * rhs.as_float().unwrap()),
            Value::Decimal(d) => Value::Decimal(d * rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Div for Value {
    type Output = Result<Value>;
    fn div(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "div")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v / rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v / rhs.as_int64().unwrap()),
            Value::Float32(v) => Value::Float32(v / rhs.as_float32().unwrap()),
            Value::Float(v) => Value::Float(v / rhs.as_float().unwrap()),
            Value::Decimal(d) => Value::Decimal(d / rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Rem for Value {
    type Output = Result<Value>;
    fn rem(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "rem")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v % rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v % rhs.as_int64().unwrap()),
            Value::Float32(v) => Value::Float32(v % rhs.as_float32().unwrap()),
            Value::Float(v) => Value::Float(v % rhs.as_float().unwrap()),
            Value::Decimal(d) => Value::Decimal(d % rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Neg for Value {
    type Output = Result<Value>;
    fn neg(self) -> Self::Output {
        check_operand(&self, "neg")?;
        Ok(match self {
            Value::Int(val) => Value::Int(-val),
            Value::Int64(val) => Value::Int64(-val),
            Value::Float32(val) => Value::Float32(-val),
            Value::Float(val) => Value::Float(-val),
            Value::Decimal(val) => Value::Decimal(-val),
            _ => unreachable!(),
        })
    }
}

impl BitAnd for Value {
    type Output = Result<Value>;
    fn bitand(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitand")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v & rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v & rhs.as_int64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl BitXor for Value {
    type Output = Result<Value>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitxor")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v ^ rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v ^ rhs.as_int64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl BitOr for Value {
    type Output = Result<Value>;
    fn bitor(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitor")?;
        Ok(match self {
            Value::Int(v) => Value::Int(v | rhs.as_int().unwrap()),
            Value::Int64(v) => Value::Int64(v | rhs.as_int64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl Neg for &Value {
    type Output = Result<Value>;

    fn neg(self) -> Self::Output {
        (self.clone()).neg()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        if self.is_any_int() && other.is_any_int() {
            return self.as_int64().unwrap() == other.as_int64().unwrap();
        }
        if self.is_any_number() && other.is_any_number() {
            return self.as_float().unwrap() == other.as_float().unwrap();
        }
        match (self, other) {
            (Null, Null) => true,
            (ObjectId(s), ObjectId(o)) => s == o,
            (Bool(s), Bool(o)) => s == o,
            (Int(s), Int(o)) => s == o,
            (Int64(s), Int64(o)) => s == o,
            (Float32(s), Float32(o)) => s == o,
            (Float(s), Float(o)) => s == o,
            (Decimal(s), Decimal(o)) => s == o,
            (String(s), String(o)) => s == o,
            (Date(s), Date(o)) => s == o,
            (DateTime(s), DateTime(o)) => s == o,
            (Array(s), Array(o)) => s == o,
            (Dictionary(s), Dictionary(o)) => s == o,
            (index_dictionary(s), index_dictionary(o)) => s == o,
            (btree_dictionary(s), btree_dictionary(o)) => s == o,
            (EnumVariant(s1, a1), EnumVariant(s2, a2)) => s1 == s2 && a1 == a2,
            _ => false,
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
        f.write_str(self.fmt_for_display().as_ref())
    }
}