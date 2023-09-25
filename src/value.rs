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
use super::pipeline::TeonPipeline;
use super::index::Index;
use super::range::TeonRange;
use super::file::TeonFile;
use super::error::Error;
use super::result::Result;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/mod.rs

/// Represents any valid Teon value. A Teon value is an extension for Teo just like Bson for
/// MongoDB.
///
#[derive(Debug, Clone)]
pub enum Value {

    /// Represents a TEON null value.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(null);
    /// ```
    Null,

    /// Represents a Teon bool.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(true);
    /// ```
    Bool(bool),

    /// Represents a Teon i32.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12_i32);
    /// ```
    I32(i32),

    /// Represents a Teon i64.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12_i64);
    /// ```
    I64(i64),

    /// Represents a Teon f32.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12.5_f32);
    /// ```
    F32(f32),

    /// Represents a Teon f64.
    ///
    /// ```
    /// # use teo_teon::teon;
    /// #
    /// let v = teon!(12.5_f64);
    /// ```
    F64(f64),

    /// Represents a Teon decimal.
    ///
    Decimal(BigDecimal),

    /// Represents a Teon object id.
    ///
    ObjectId(ObjectId),

    /// Represents a Teon string.
    ///
    String(String),

    /// Represents a Teon date.
    ///
    Date(NaiveDate),

    /// Represents a Teon datetime.
    ///
    DateTime(DateTime<Utc>),

    /// Represents a Teon array.
    ///
    Vec(Vec<Value>),

    /// Represents a Teon hashmap.
    ///
    HashMap(HashMap<String, Value>),

    /// Represents a Teon btreemap.
    ///
    BTreeMap(BTreeMap<String, Value>),

    /// Represents a Teon btreemap.
    ///
    IndexMap(IndexMap<String, Value>),

    /// Represents a Teon range.
    ///
    Range(TeonRange),

    /// Represents a Teon tuple.
    ///
    Tuple(Vec<Value>),

    /// Represents a Teon pipeline.
    ///
    Pipeline(TeonPipeline),

    /// Raw enum choice.
    ///
    RawEnumVariant(String, Option<Vec<(Option<String>, Value)>>),

    /// Raw option choice
    ///
    RawOptionVariant(u32),

    /// Regular expression
    ///
    RegExp(Regex),

    /// Represents a file.
    ///
    File(TeonFile),
}

impl Value {

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    pub fn is_hashmap(&self) -> bool {
        self.as_hashmap().is_some()
    }

    pub fn as_hashmap(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::HashMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_hashmap_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::HashMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_btreemap(&self) -> bool {
        self.as_btreemap().is_some()
    }

    pub fn as_btreemap(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::BTreeMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_btreemap_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match self {
            Value::BTreeMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_indexmap(&self) -> bool {
        self.as_indexmap().is_some()
    }

    pub fn as_indexmap(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::IndexMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_indexmap_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::IndexMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_vec(&self) -> bool {
        self.as_vec().is_some()
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Vec(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn into_vec(self) -> Option<Vec<Value>> {
        match self {
            Value::Vec(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Vec(vec) => Some(vec),
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

    pub fn str_from_string_or_raw_enum_variant(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::RawEnumVariant(s, _) => Some(s),
            _ => None,
        }
    }

    pub fn is_i(&self) -> bool {
        match *self {
            Value::I32(_) | Value::I64(_) => true,
            _ => false,
        }
    }

    pub fn is_f(&self) -> bool {
        match *self {
            Value::F32(_) | Value::F64(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        self.is_i() || self.is_f()
    }

    pub fn is_i32(&self) -> bool {
        match *self {
            Value::I32(_) => true,
            _ => false,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Value::I32(v) => Some(v),
            Value::I64(v) => Some(v as i32),
            Value::F32(f) => Some(f as i32),
            Value::F64(f) => Some(f as i32),
            _ => None
        }
    }

    pub fn is_i64(&self) -> bool {
        match *self {
            Value::I64(_) => true,
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::I32(v) => Some(v as i64),
            Value::I64(v) => Some(v),
            Value::F32(f) => Some(f as i64),
            Value::F64(f) => Some(f as i64),
            _ => None
        }
    }

    pub fn is_f32(&self) -> bool {
        match *self {
            Value::F32(_v) => true,
            _ => false,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            Value::I32(v) => Some(v as f32),
            Value::I64(v) => Some(v as f32),
            Value::F32(v) => Some(v),
            Value::F64(v) => Some(v as f32),
            _ => None
        }
    }

    pub fn is_f64(&self) -> bool {
        match *self {
            Value::F64(_v) => true,
            _ => false,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::I32(v) => Some(v as f64),
            Value::I64(v) => Some(v as f64),
            Value::F32(v) => Some(v as f64),
            Value::F64(v) => Some(v as f64),
            _ => None
        }
    }

    pub fn is_decimal(&self) -> bool {
        match *self {
            Value::Decimal(_) => true,
            _ => false,
        }
    }

    pub fn as_decimal(&self) -> Option<BigDecimal> {
        match self {
            Value::Decimal(v) => Some(v.clone()),
            _ => None
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Value::I32(n) => Some(*n as usize),
            Value::I64(n) => Some(*n as usize),
            _ => None
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

    pub fn is_object_id(&self) -> bool {
        self.as_object_id().is_some()
    }

    pub fn as_object_id(&self) -> Option<&ObjectId> {
        match self {
            Value::ObjectId(o) => Some(o),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn is_raw_enum_variant(&self) -> bool {
        self.as_raw_enum_variant().is_some()
    }

    pub fn as_raw_enum_variant(&self) -> Option<&str> {
        match self {
            Value::RawEnumVariant(s, _) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn is_raw_option_variant(&self) -> bool {
        self.as_raw_option_variant().is_some()
    }

    pub fn as_raw_option_variant(&self) -> Option<u32> {
        match self {
            Value::RawOptionVariant(o) => Some(*o),
            _ => None,
        }
    }

    pub fn is_range(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_range(&self) -> Option<&TeonRange> {
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

    pub fn is_file(&self) -> bool {
        self.as_file().is_some()
    }

    pub fn as_file(&self) -> Option<&TeonFile> {
        match self {
            Value::File(f) => Some(f),
            _ => None,
        }
    }

    pub fn is_pipeline(&self) -> bool {
        self.as_pipeline().is_some()
    }

    pub fn as_pipeline(&self) -> Option<&TeonPipeline> {
        match self {
            Value::Pipeline(p) => Some(p),
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

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    pub fn recip(&self) -> f64 {
        match self {
            Value::I32(n) => (*n as f64).recip(),
            Value::I64(n) => (*n as f64).recip(),
            Value::F32(n) => (*n as f64).recip(),
            Value::F64(n) => (*n as f64).recip(),
            Value::Decimal(_n) => panic!("decimal div todo"),
            _ => panic!()
        }
    }

    pub fn fmt_for_display(&self) -> Cow<str> {
        match self {
            Value::Null => Cow::Borrowed("null"),
            Value::Bool(v) => if *v { Cow::Borrowed("true") } else { Cow::Borrowed("false") },
            Value::I32(i) => Cow::Owned(i.to_string()),
            Value::I64(i) => Cow::Owned(i.to_string()),
            Value::F32(f) => Cow::Owned(f.to_string()),
            Value::F64(f) => Cow::Owned(f.to_string()),
            Value::Decimal(d) => Cow::Owned(format!("Decimal(\"{}\")", d.to_string())),
            Value::ObjectId(o) => Cow::Owned(format!("ObjectId(\"{}\")", o.to_hex())),
            Value::String(s) => Cow::Owned(format!("\"{}\"", s.replace("\"", "\\\""))),
            Value::Date(d) => Cow::Owned(format!("Date(\"{}\")", d.to_string())),
            Value::DateTime(dt) => Cow::Owned(format!("DateTime(\"{}\")", dt.to_rfc3339_opts(SecondsFormat::Millis, true))),
            Value::Vec(v) => Cow::Owned("[".to_string() + v.iter().map(|v| v.fmt_for_display()).join(", ").as_str() + "]"),
            Value::HashMap(m) => Cow::Owned("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", v.fmt_for_display())).join(", ").as_str() + "}"),
            Value::BTreeMap(m) => Cow::Owned("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", v.fmt_for_display())).join(", ").as_str() + "}"),
            Value::IndexMap(m) => Cow::Owned("{".to_string() + m.iter().map(|(k, v)| format!("\"{k}\": {}", v.fmt_for_display())).join(", ").as_str() + "}"),
            Value::Range(_) => unreachable!(),
            Value::Tuple(v) => Cow::Owned("(".to_string() + v.iter().map(|v| v.fmt_for_display()).join(", ").as_str() + ")"),
            Value::Pipeline(_) => unreachable!(),
            Value::RawEnumVariant(v, _) => Cow::Owned(format!(".{}", v.as_str())),
            Value::RawOptionVariant(_) => unreachable!(),
            Value::RegExp(_) => unreachable!(),
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
            (I32(s), I32(o)) => s.partial_cmp(o),
            (I64(s), I64(o)) => s.partial_cmp(o),
            (F32(s), F32(o)) => s.partial_cmp(o),
            (F64(s), F64(o)) => s.partial_cmp(o),
            (Decimal(s), Decimal(o)) => s.partial_cmp(o),
            (String(s), String(o)) => s.partial_cmp(o),
            (Date(s), Date(o)) => s.partial_cmp(o),
            (DateTime(s), DateTime(o)) => s.partial_cmp(o),
            (Vec(s), Vec(o)) => s.partial_cmp(o),
            (HashMap(_s), HashMap(_o)) => None,
            (BTreeMap(_s), BTreeMap(_o)) => None,
            _ => None,
        }
    }
}

fn check_operand(lhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_number() {
        return Err(Error::new(format!("{}: operand is not number", name)));
    }
    Ok(())
}

fn check_operands(lhs: &Value, rhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_number() {
        return Err(Error::new(format!("{}: lhs is not number", name)));
    }
    if !rhs.is_number() {
        return Err(Error::new(format!("{}: rhs is not number", name)));
    }
    Ok(())
}

fn check_operands_int(lhs: &Value, rhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_i() {
        return Err(Error::new(format!("{}: lhs is not number", name)));
    }
    if !rhs.is_i() {
        return Err(Error::new(format!("{}: rhs is not number", name)));
    }
    Ok(())
}

impl Add for Value {
    type Output = Result<Value>;
    fn add(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "add")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v + rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v + rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v + rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v + rhs.as_f64().unwrap()),
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
            Value::I32(v) => Value::I32(v - rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v - rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v - rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v - rhs.as_f64().unwrap()),
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
            Value::I32(v) => Value::I32(v * rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v * rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v * rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v * rhs.as_f64().unwrap()),
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
            Value::I32(v) => Value::I32(v / rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v / rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v / rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v / rhs.as_f64().unwrap()),
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
            Value::I32(v) => Value::I32(v % rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v % rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v % rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v % rhs.as_f64().unwrap()),
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
            Value::I32(val) => Value::I32(-val),
            Value::I64(val) => Value::I64(-val),
            Value::F32(val) => Value::F32(-val),
            Value::F64(val) => Value::F64(-val),
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
            Value::I32(v) => Value::I32(v & rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v & rhs.as_i64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl BitXor for Value {
    type Output = Result<Value>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitxor")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v ^ rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v ^ rhs.as_i64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl BitOr for Value {
    type Output = Result<Value>;
    fn bitor(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitor")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v | rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v | rhs.as_i64().unwrap()),
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
        if self.is_i() && other.is_i() {
            return self.as_i64().unwrap() == other.as_i64().unwrap();
        }
        if self.is_number() && other.is_number() {
            return self.as_f64().unwrap() == other.as_f64().unwrap();
        }
        match (self, other) {
            (Null, Null) => true,
            (ObjectId(s), ObjectId(o)) => s == o,
            (Bool(s), Bool(o)) => s == o,
            (I32(s), I32(o)) => s == o,
            (I64(s), I64(o)) => s == o,
            (F32(s), F32(o)) => s == o,
            (F64(s), F64(o)) => s == o,
            (Decimal(s), Decimal(o)) => s == o,
            (String(s), String(o)) => s == o,
            (Date(s), Date(o)) => s == o,
            (DateTime(s), DateTime(o)) => s == o,
            (Vec(s), Vec(o)) => s == o,
            (HashMap(s), HashMap(o)) => s == o,
            (IndexMap(s), IndexMap(o)) => s == o,
            (BTreeMap(s), BTreeMap(o)) => s == o,
            (RawEnumVariant(s1, a1), RawEnumVariant(s2, a2)) => s1 == s2 && a1 == a2,
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