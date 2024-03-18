use indexmap::indexmap;
use teo_result::Error;
use crate::Value;

impl From<Error> for Value {

    fn from(value: Error) -> Value {
        Value::from(&value)
    }
}

impl From<&Error> for Value {

    fn from(value: &Error) -> Value {
        let errors = value.errors.as_ref().map(|f| {
            let mut result = indexmap! {};
            for (k, v) in f {
                result.insert(k.to_string(), Value::String(v.to_string()));
            }
            Value::Dictionary(result)
        }) ;
        let mut retval = Value::Dictionary(indexmap! {
            "type".to_string() => Value::String(value.inferred_title().to_string()),
            "message".to_string() => Value::String(value.message.clone()),
        });
        if errors.is_some() {
            retval.as_dictionary_mut().unwrap().insert("errors".to_owned(), errors.unwrap());
        }
        retval
    }
}
