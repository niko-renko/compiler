use super::*;

impl Write for Get {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "getelt(")?;
        // self.ptr.write(writer, classes, function)?;
        // write!(writer, ", ")?;
        // self.offset.write(writer, classes, function)?;
        // write!(writer, ")")
    }
}
