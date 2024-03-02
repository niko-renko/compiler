use super::*;

impl Write for Operator {
    fn write(&self, writer: &mut Writer, _: &Classes, _: &FunctionContext) {
        writer.write_code(&self.to_string());
    }
}

impl Write for Op {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        self.get_left().write(writer, classes, function);
        writer.write_code(" ");
        self.get_operator().write(writer, classes, function);
        writer.write_code(" ");
        self.get_right().write(writer, classes, function);
    }
}
