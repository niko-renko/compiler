use super::*;

mod branch;
mod fail;
mod jump;
mod r#return;

pub use branch::Branch;
pub use fail::{Fail, FailReason};
pub use jump::Jump;
pub use r#return::Return;

#[derive(Clone, Copy)]
pub enum ControlTransfer {
    Return(Return),
    Branch(Branch),
    Jump(Jump),
    Fail(Fail),
}

impl PlacesRead for ControlTransfer {
    fn places_read(&self) -> Vec<Place> {
        match self {
            ControlTransfer::Return(cf) => cf.places_read(),
            ControlTransfer::Branch(cf) => cf.places_read(),
            ControlTransfer::Jump(cf) => cf.places_read(),
            ControlTransfer::Fail(cf) => cf.places_read(),
        }
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        match self {
            ControlTransfer::Return(cf) => cf.places_read_mut(),
            ControlTransfer::Branch(cf) => cf.places_read_mut(),
            ControlTransfer::Jump(cf) => cf.places_read_mut(),
            ControlTransfer::Fail(cf) => cf.places_read_mut(),
        }
    }
}

impl Write for ControlTransfer {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        match self {
            ControlTransfer::Return(cf) => cf.write(writer, classes, function),
            ControlTransfer::Branch(cf) => cf.write(writer, classes, function),
            ControlTransfer::Jump(cf) => cf.write(writer, classes, function),
            ControlTransfer::Fail(cf) => cf.write(writer, classes, function),
        }
    }
}
