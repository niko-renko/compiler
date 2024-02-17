use super::*;

mod named;
mod r#static;
mod temp;

pub use named::Named;
use r#static::Static;
pub use temp::Temp;

#[derive(Clone, Copy)]
pub enum Place {
    Local(Named),
    Temp(Temp),
    Static(Static),
}

impl Into<PlaceValue> for Place {
    fn into(self) -> PlaceValue {
        PlaceValue::Place(self)
    }
}
