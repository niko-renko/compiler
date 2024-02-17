use super::*;

#[derive(Clone, Copy)]
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
}

impl Into<PlaceValue> for Value {
    fn into(self) -> PlaceValue {
        PlaceValue::Value(self)
    }
}

impl Write for Value {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        _: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "{}", self.0)
    }
}
