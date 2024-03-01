use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(usize);

impl Value {
    pub fn from(value: usize) -> Value {
        let value = value << 2;
        let value = value + 1;
        Value(value)
    }

    pub fn from_raw(value: usize) -> Value {
        Value(value)
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
