use crate::ast::{Class, Declaration, Function, Local, Name, Statement, Type, AST};

mod classes;
mod functions;
mod traits;

pub use classes::Classes;
pub use functions::{FunctionContext, Functions};
pub use traits::Extract;
