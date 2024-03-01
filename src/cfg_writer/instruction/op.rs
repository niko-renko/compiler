use super::*;

impl Write for Operator {
    fn write(
        &self,
        writer: &mut Writer,
        _: &Classes,
        _: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // write!(writer, " {} ", self.get_char())
        Ok(())
    }
}

impl Write for Op {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // self.left.write(writer, classes, function)?;
        // self.operator.write(writer, classes, function)?;
        // self.right.write(writer, classes, function)
    }
}
