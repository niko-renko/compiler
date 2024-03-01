use super::*;

impl Write for Branch {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "if ")?;
        // self.condition.write(writer, classes, function)?;
        // write!(writer, " then ")?;
        // self.true_label.write(writer, classes, function)?;
        // write!(writer, " else ")?;
        // self.false_label.write(writer, classes, function)
    }
}
