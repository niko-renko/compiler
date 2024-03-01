use super::*;

impl Write for Jump {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "jump ")?;
        // self.0.write(writer, classes, function)
    }
}
