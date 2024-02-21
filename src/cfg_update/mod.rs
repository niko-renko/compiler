use crate::ast;
use crate::ast_extract::{Classes, Function};
use crate::cfg::{Instruction, InstructionHash, Label, Place, PlacesRead, CFG};
use crate::cfg_extract::Assign;
use crate::Extract;

mod build;
mod peephole;
mod ssa;
mod traits;
mod vn;

pub use build::Build;
pub use peephole::Peephole;
pub use ssa::SSA;
pub use traits::Update;
pub use vn::VN;
