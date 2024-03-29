use std::hash::Hash;

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

impl Used for Instruction {
    fn used(&self) -> Vec<PlaceValue> {
        match self {
            Instruction::Alloc(i) => i.used(),
            Instruction::Call(i) => i.used(),
            Instruction::Get(i) => i.used(),
            Instruction::Alias(i) => i.used(),
            Instruction::Op(i) => i.used(),
            Instruction::Phi(i) => i.used(),
            Instruction::Print(i) => i.used(),
            Instruction::Set(i) => i.used(),
        }
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        match self {
            Instruction::Alloc(i) => i.used_mut(),
            Instruction::Call(i) => i.used_mut(),
            Instruction::Get(i) => i.used_mut(),
            Instruction::Alias(i) => i.used_mut(),
            Instruction::Op(i) => i.used_mut(),
            Instruction::Phi(i) => i.used_mut(),
            Instruction::Print(i) => i.used_mut(),
            Instruction::Set(i) => i.used_mut(),
        }
    }
}

impl InstructionHash for Instruction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Instruction::Alloc(i) => i.hash(state),
            Instruction::Call(i) => i.hash(state),
            Instruction::Get(i) => i.hash(state),
            Instruction::Alias(i) => i.hash(state),
            Instruction::Op(i) => i.hash(state),
            Instruction::Phi(i) => i.hash(state),
            Instruction::Print(i) => i.hash(state),
            Instruction::Set(i) => i.hash(state),
        }
    }

    fn get_constant(&self, vn: &HashMap<u64, PlaceValue>) -> Option<Value> {
        match self {
            Instruction::Alloc(i) => i.get_constant(vn),
            Instruction::Call(i) => i.get_constant(vn),
            Instruction::Get(i) => i.get_constant(vn),
            Instruction::Alias(i) => i.get_constant(vn),
            Instruction::Op(i) => i.get_constant(vn),
            Instruction::Phi(i) => i.get_constant(vn),
            Instruction::Print(i) => i.get_constant(vn),
            Instruction::Set(i) => i.get_constant(vn),
        }
    }
}
