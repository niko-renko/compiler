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
    None,
}

impl Into<PlaceValue> for Place {
    fn into(self) -> PlaceValue {
        PlaceValue::Place(self)
    }
}

impl Write for Place {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        match self {
            // Place::Local(p) => p.write(writer, classes, function),
            Place::Temp(p) => p.write(writer, classes, function),
            // Place::Static(p) => p.write(writer, classes, function),
            Place::None => Ok(()),
            _ => Ok(()),
        }
    }
}