use super::*;

mod alias;
mod alloc;
mod call;
mod get;
mod op;
mod phi;
mod print;
mod set;

pub use alias::Alias;
pub use alloc::Alloc;
pub use call::Call;
pub use get::Get;
pub use op::{Op, Operator};
pub use phi::Phi;
pub use print::Print;
pub use set::Set;

pub enum Instruction {
    Alloc(Alloc),
    Call(Call),
    Get(Get),
    Alias(Alias),
    Op(Op),
    Phi(Phi),
    Print(Print),
    Set(Set),
}
