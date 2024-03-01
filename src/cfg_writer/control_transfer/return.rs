use super::*;

impl Write for Return {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // write!(writer, "ret ")?;
        // self.0.write(writer, classes, function)
        Ok(())
    }
}
