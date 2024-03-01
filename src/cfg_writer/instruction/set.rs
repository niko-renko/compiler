use super::*;

impl Write for Set {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "setelt(")?;
        // self.ptr.write(writer, classes, function)?;
        // write!(writer, ", ")?;
        // self.offset.write(writer, classes, function)?;
        // write!(writer, ", ")?;
        // self.value.write(writer, classes, function)?;
        // write!(writer, ")")
    }
}
