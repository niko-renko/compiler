use crate::cfg::{
    Alias, Instruction, InstructionHash, Label, Operator, Place, PlaceValue, Used, Value, CFG,
};
use crate::cfg_extract::Assign;
use crate::Extract;

mod peephole;
mod ssa;
mod traits;
mod vn;

pub use peephole::Peephole;
pub use ssa::SSA;
pub use traits::Update;
pub use vn::VN;
