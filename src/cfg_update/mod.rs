use crate::cfg::*;
use crate::cfg_extract::{Assign, Dom};
use crate::traits::{Extract, Update};

mod peephole;
mod ssa;
mod vn;

pub use peephole::Peephole;
pub use ssa::SSA;
pub use vn::VN;
