use super::*;

impl Write for Call {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        //  write!(writer, "call(")?;
        //  self.code.write(writer, classes, function)?;
        //  write!(writer, ", ")?;
        //  self.object.write(writer, classes, function)?;
        //  for arg in &self.args {
        //      write!(writer, ", ")?;
        //      arg.write(writer, classes, function)?;
        //  }
        //  write!(writer, ")")
    }
}
