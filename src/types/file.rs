use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use maplit::hashset;
use serde_json::{Value as JsonValue};
use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub filepath: String,
    pub content_type: Option<String>,
    pub filename: String,
    pub filename_ext: Option<String>,
}

impl Display for File {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("File(\"")?;
        f.write_str(&self.filepath.as_str().replace("\"", "\\\""))?;
        f.write_str("\")")
    }
}

impl TryFrom<&JsonValue> for File {

    type Error = Error;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        if let Some(object) = value.as_object() {
            let keys_set: HashSet<&str> = object.keys().map(|k| k.as_str()).collect();
            let difference: HashSet<&str> = keys_set.difference(&hashset!{"filepath", "contentType", "filename", "filenameExt"}).map(|s| *s).collect();
            if !difference.is_empty() {
                return Err(Error::new(format!("Connot convert json value to file, unexpected key {}", difference.iter().map(|k| format!("`{}`", *k)).join(", "))));
            }
            Ok(Self {
                filepath: if let Some(filepath) = object.get("filepath") {
                    if let Some(filepath) = filepath.as_str() {
                        filepath.to_owned()
                    } else {
                        Err(Error::new(format!("Cannot convert json value to file, invalid value at `filepath`, expect string")))?
                    }
                } else {
                    Err(Error::new(format!("Cannot convert json value to file, missing key `filepath`")))?
                },
                content_type: if let Some(content_type) = object.get("contentType") {
                    if let Some(content_type) = content_type.as_str() {
                        Some(content_type.to_owned())
                    } else {
                        Err(Error::new(format!("Cannot convert json value to file, invalid value at `contentType`, expect string")))?
                    }
                } else {
                    None
                },
                filename: if let Some(filename) = object.get("filename") {
                    if let Some(filename) = filename.as_str() {
                        filename.to_owned()
                    } else {
                        Err(Error::new(format!("Cannot convert json value to file, invalid value at `filename`, expect string")))?
                    }
                } else {
                    Err(Error::new(format!("Cannot convert json value to file, missing key `filename`")))?
                },
                filename_ext: if let Some(filename_ext) = object.get("filenameExt") {
                    if let Some(filename_ext) = filename_ext.as_str() {
                        Some(filename_ext.to_owned())
                    } else {
                        Err(Error::new(format!("Cannot convert json value to file, invalid value at `filenameExt`, expect string")))?
                    }
                } else {
                    None
                },
            })
        } else {
            Err(Error::new(format!("Cannot convert json value to file, value `{}` is not object", value)))
        }
    }
}
