use super::*;

impl Write for Alloc {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("alloc(");
        self.get_size().write(writer, classes, function);
        writer.write_code(")");
    }
}
