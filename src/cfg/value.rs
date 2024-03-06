use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(usize);

impl Value {
    pub fn from(value: usize) -> Self {
        Self(value)
    }

    pub fn get_value(&self) -> usize {
        self.0
    }
}

impl Into<PlaceValue> for Value {
    fn into(self) -> PlaceValue {
        PlaceValue::Value(self)
    }
}
