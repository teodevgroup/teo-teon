use std::ops;
use super::value::Value;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/index.rs

pub trait Index {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
}

impl Index for usize {

    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }
}

impl Index for str {

    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Dictionary(map) => map.get(self),
            Value::BTreeDictionary(map) => map.get(self),
            Value::IndexDictionary(map) => map.get(self),
            _ => None,
        }
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Dictionary(map) => map.get_mut(self),
            Value::BTreeDictionary(map) => map.get_mut(self),
            Value::IndexDictionary(map) => map.get_mut(self),
            _ => None,
        }
    }
}

impl Index for String {

    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
}

impl<'a, T> Index for &'a T where T: ?Sized + Index, {

    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
}

impl<I> ops::Index<I> for Value where I: Index {

    type Output = Value;

    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Value where I: Index {

    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }
}
