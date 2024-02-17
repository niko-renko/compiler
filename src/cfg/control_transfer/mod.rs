use super::*;

mod branch;
mod fail;
mod jump;
mod r#return;

pub use branch::Branch;
pub use fail::{Fail, FailReason};
use jump::Jump;
pub use r#return::Return;

pub enum ControlTransfer {
    Return(Return),
    Branch(Branch),
    Jump(Jump),
    Fail(Fail),
}

impl ControlTransfer {}
