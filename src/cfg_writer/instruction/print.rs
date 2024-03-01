use super::*;

impl Write for Print {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "print(")?;
        // self.value.write(writer, classes, function)?;
        // write!(writer, ")")
    }
}
