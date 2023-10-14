use crate::value::Value;

impl From<f32> for Value {

    fn from(v: f32) -> Self {
        Value::Float32(v)
    }
}

impl From<&f32> for Value {

    fn from(v: &f32) -> Self {
        Value::Float32(*v)
    }
}

impl From<f32> for &Value {

    fn from(v: f32) -> Self {
        &Value::Float32(v)
    }
}

impl From<&f32> for &Value {

    fn from(v: &f32) -> Self {
        &Value::Float32(*v)
    }
}

impl From<Option<f32>> for Value {

    fn from(v: Option<f32>) -> Self {
        match v {
            Some(b) => Value::Float32(b),
            None => Value::Null,
        }
    }
}

impl From<Option<&f32>> for Value {

    fn from(v: Option<&f32>) -> Self {
        match v {
            Some(b) => Value::Float32(*b),
            None => Value::Null,
        }
    }
}

// impl From<Option<f32>> for &Value {

//     fn from(v: Option<f32>) -> Self {
//         match v {
//             Some(b) => &Value::Float32(b),
//             None => &Value::Null,
//         }
//     }
// }

// impl From<Option<&f32>> for &Value {

//     fn from(v: Option<&f32>) -> Self {
//         match v {
//             Some(b) => &Value::Float32(*b),
//             None => &Value::Null,
//         }
//     }
// }
