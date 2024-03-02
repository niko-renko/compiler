use super::*;

impl Write for Call {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("call(");
        self.get_code().write(writer, classes, function);
        writer.write_code(", ");
        self.get_object().write(writer, classes, function);
        for arg in self.get_args() {
            writer.write_code(", ");
            arg.write(writer, classes, function);
        }
        writer.write_code(")");
    }
}
