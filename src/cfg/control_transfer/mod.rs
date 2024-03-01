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

impl Used for ControlTransfer {
    fn used(&self) -> Vec<PlaceValue> {
        match self {
            ControlTransfer::Return(cf) => cf.used(),
            ControlTransfer::Branch(cf) => cf.used(),
            ControlTransfer::Jump(cf) => cf.used(),
            ControlTransfer::Fail(cf) => cf.used(),
        }
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        match self {
            ControlTransfer::Return(cf) => cf.used_mut(),
            ControlTransfer::Branch(cf) => cf.used_mut(),
            ControlTransfer::Jump(cf) => cf.used_mut(),
            ControlTransfer::Fail(cf) => cf.used_mut(),
        }
    }
}
