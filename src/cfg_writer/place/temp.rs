use super::*;

impl Write for Temp {
    fn write(
        &self,
        writer: &mut Writer,
        _: &Classes,
        _: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // write!(writer, "%{}", self.0)
        Ok(())
    }
}
