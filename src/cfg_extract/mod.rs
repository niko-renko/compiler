use crate::cfg::{Label, Place, PlaceValue, Used, CFG};

mod assign;
mod dom;
mod traits;

pub use assign::Assign;
pub use dom::Dom;
pub use traits::Extract;
