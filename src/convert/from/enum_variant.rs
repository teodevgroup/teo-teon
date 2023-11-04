use crate::types::enum_variant::EnumVariant;
use crate::Value;

impl From<EnumVariant> for Value {

    fn from(value: EnumVariant) -> Self {
        Self::EnumVariant(value)
    }
}

impl From<&EnumVariant> for Value {

    fn from(value: &EnumVariant) -> Self {
        Self::EnumVariant(value.clone())
    }
}