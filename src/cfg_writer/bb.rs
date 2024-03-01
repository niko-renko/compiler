use super::*;

impl Write for BB {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // for (place, instruction) in &self.instructions {
        //     if let Place::None = place {
        //     } else {
        //         place.write(writer, classes, function)?;
        //         write!(writer, " = ")?;
        //     }
        //     instruction.write(writer, classes, function)?;
        //     writeln!(writer)?;
        // }

        // self.end.write(writer, classes, function)?;
        // write!(writer, "\n\n")
        Ok(())
    }
}
