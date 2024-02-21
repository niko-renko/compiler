use crate::cfg::{Label, Place, PlaceValue, Used, CFG};
use crate::Extract;

mod assign;
mod dom;

pub use assign::Assign;
pub use dom::Dom;
