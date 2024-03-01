use super::*;

impl Write for Alias {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // self.0.write(writer, classes, function)
        Ok(())
    }
}
