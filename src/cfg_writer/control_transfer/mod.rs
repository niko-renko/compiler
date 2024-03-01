use super::*;

mod branch;
mod fail;
mod jump;
mod r#return;

impl Write for ControlTransfer {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        match self {
            ControlTransfer::Return(cf) => cf.write(writer, classes, function),
            ControlTransfer::Branch(cf) => cf.write(writer, classes, function),
            ControlTransfer::Jump(cf) => cf.write(writer, classes, function),
            ControlTransfer::Fail(cf) => cf.write(writer, classes, function),
        }
    }
}
