use super::*;

impl Write for Branch {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("if ");
        self.get_condition().write(writer, classes, function);
        writer.write_code(" then ");
        self.get_true().write(writer, classes, function);
        writer.write_code(" else ");
        self.get_false().write(writer, classes, function);
    }
}
