use crate::types::file::File;
use crate::Value;

impl From<File> for Value {

    fn from(value: File) -> Self {
        Self::File(value)
    }
}

impl From<&File> for Value {

    fn from(value: &File) -> Self {
        Self::File(value.clone())
    }
}