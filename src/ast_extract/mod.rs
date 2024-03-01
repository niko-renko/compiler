use crate::ast::{Class, Declaration, Function, Local, Name, Statement, Type, AST};
use crate::Extract;

mod classes;
mod functions;

pub use classes::Classes;
pub use functions::{FunctionContext, Functions};
