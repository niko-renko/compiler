use super::*;

impl Write for Alloc {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "alloc(")?;
        // self.size.write(writer, classes, function)?;
        // write!(writer, ")")
    }
}
