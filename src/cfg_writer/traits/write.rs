use super::*;

pub trait Write {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext);
}
