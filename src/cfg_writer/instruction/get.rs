use super::*;

impl Write for Get {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("getelt(");
        self.get_ptr().write(writer, classes, function);
        writer.write_code(", ");
        self.get_offset().write(writer, classes, function);
        writer.write_code(")");
    }
}
