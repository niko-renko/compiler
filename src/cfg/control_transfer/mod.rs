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
