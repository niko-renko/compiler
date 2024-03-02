use crate::cfg::*;
use crate::cfg_extract::{Assign, Extract};

mod peephole;
mod ssa;
mod traits;
mod vn;

pub use peephole::Peephole;
pub use ssa::SSA;
pub use traits::Update;
pub use vn::VN;
