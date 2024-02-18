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

impl PlacesRead for Instruction {
    fn places_read(&self) -> Vec<Place> {
        match self {
            // Instruction::Alloc(alloc) => alloc.places_read(),
            // Instruction::Call(call) => call.places_read(),
            // Instruction::Get(get) => get.places_read(),
            Instruction::Alias(i) => i.places_read(),
            Instruction::Op(i) => i.places_read(),
            Instruction::Phi(i) => i.places_read(),
            Instruction::Print(i) => i.places_read(),
            // Instruction::Set(set) => set.places_read(),
            _ => Vec::new(),
        }
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        match self {
            // Instruction::Alloc(alloc) => alloc.places_read_mut(),
            // Instruction::Call(call) => call.places_read_mut(),
            // Instruction::Get(get) => get.places_read_mut(),
            Instruction::Alias(i) => i.places_read_mut(),
            Instruction::Op(i) => i.places_read_mut(),
            Instruction::Phi(i) => i.places_read_mut(),
            Instruction::Print(i) => i.places_read_mut(),
            // Instruction::Set(set) => set.places_read_mut(),
            _ => Vec::new(),
        }
    }
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
            Instruction::Phi(i) => i.write(writer, classes, function),
            Instruction::Print(i) => i.write(writer, classes, function),
            // Instruction::Set(set) => set.write(writer, classes, function),
            _ => Ok(()),
        }
    }
}
