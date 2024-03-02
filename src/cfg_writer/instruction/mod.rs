use super::*;

mod alias;
mod alloc;
mod call;
mod get;
mod op;
mod phi;
mod print;
mod set;

impl Write for Instruction {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        match self {
            Instruction::Alloc(i) => i.write(writer, classes, function),
            Instruction::Call(i) => i.write(writer, classes, function),
            Instruction::Get(i) => i.write(writer, classes, function),
            Instruction::Alias(i) => i.write(writer, classes, function),
            Instruction::Op(i) => i.write(writer, classes, function),
            Instruction::Phi(i) => i.write(writer, classes, function),
            Instruction::Print(i) => i.write(writer, classes, function),
            Instruction::Set(i) => i.write(writer, classes, function),
        }
    }
}
