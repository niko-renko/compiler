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

impl Write for Instruction {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        match self {
            // Instruction::Alloc(alloc) => alloc.write(writer, classes, function),
            // Instruction::Call(call) => call.write(writer, classes, function),
            // Instruction::Get(get) => get.write(writer, classes, function),
            Instruction::Alias(i) => i.write(writer, classes, function),
            Instruction::Op(i) => i.write(writer, classes, function),
            // Instruction::Phi(phi) => phi.write(writer, classes, function),
            Instruction::Print(i) => i.write(writer, classes, function),
            // Instruction::Set(set) => set.write(writer, classes, function),
            _ => Ok(()),
        }
    }
}