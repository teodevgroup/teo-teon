use crate::value::Value;

impl From<i32> for Value {

    fn from(v: i32) -> Self {
        Value::Int(v)
    }
}

impl From<&i32> for Value {

    fn from(v: &i32) -> Self {
        Value::Int(*v)
    }
}

impl From<i32> for &Value {

    fn from(v: i32) -> Self {
        &Value::Int(v)
    }
}

impl From<&i32> for &Value {

    fn from(v: &i32) -> Self {
        &Value::Int(*v)
    }
}

impl From<Option<i32>> for Value {

    fn from(v: Option<i32>) -> Self {
        match v {
            Some(b) => Value::Int(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&i32>> for Value {

    fn from(v: Option<&i32>) -> Self {
        match v {
            Some(b) => Value::Int(*b),
            None => Value::Null,
        }
    }
}

// impl From<Option<i32>> for &Value {

//     fn from(v: Option<i32>) -> Self {
//         match v {
//             Some(b) => &Value::Int(b),
//             None => &Value::Null,
//         }
//     }
// }

// impl From<Option<&i32>> for &Value {

//     fn from(v: Option<&i32>) -> Self {
//         match v {
//             Some(b) => &Value::Int(*b),
//             None => &Value::Null,
//         }
//     }
// }

