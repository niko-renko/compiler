use super::*;

impl Write for BB {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        for (place, instruction) in self.get_instructions() {
            if let Place::None = place {
            } else {
                place.write(writer, classes, function);
                writer.write_code(" = ");
            }
            instruction.write(writer, classes, function);
            writer.write_code("\n");
        }

        self.get_end().write(writer, classes, function);
        writer.write_code("\n\n");
    }
}
