use super::*;

impl Write for Phi {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "phi(")?;
        // let mut first = true;
        // for (named, label) in &self.0 {
        //     if !first {
        //         write!(writer, ", ")?;
        //     }
        //     first = false;
        //     label.write(writer, classes, function)?;
        //     write!(writer, ", ")?;
        //     named.write(writer, classes, function)?;
        // }
        // write!(writer, ")")
    }
}
